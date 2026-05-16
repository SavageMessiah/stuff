#include <string.h>
#define SOKOL_IMPL
#define SOKOL_GLCORE
#include "sokol_app.h"
#include "sokol_gfx.h"
#include "sokol_glue.h"
#include "sokol_gp.h"
#include "sokol_log.h"
#include <errno.h>
#include <fcntl.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <unistd.h>

static char *wad;
struct header {
  char id[4];
  uint32_t num;
  uint32_t ofs;
};

struct dir {
  uint32_t pos;
  uint32_t size;
  char name[8];
};

struct vertex {
  int16_t x;
  int16_t y;
};

struct line {
  int16_t v1;
  int16_t v2;
  int16_t flags;
  int16_t special;
  int16_t tag;
  int16_t sidenum[2];
};

struct level {
  char name[8];
  struct vertex bounds[2];
  uint32_t numL;
  sgp_line *lines;
};

static struct level levels[36] = {0};
static int numLevels = 0;
static int curLevel = 0;

static void level(struct dir *dir) {
  struct level *l = &levels[numLevels++];
  printf("level: %.8s\n", dir->name);
  memcpy(l->name, dir->name, 8);

  struct vertex *vs;
  int numV;
  struct line *ls;

  for (dir++; dir->size != 0; dir++) {
    printf("dir pos %d size %d name %.8s\n", dir->pos, dir->size, dir->name);
    if (!strncmp("VERTEXES", dir->name, 8)) {
      numV = dir->size / sizeof(struct vertex);
      vs = (struct vertex *)(wad + dir->pos);
    }
    if (!strncmp("LINEDEFS", dir->name, 8)) {
      l->numL = dir->size / sizeof(struct line);
      ls = (struct line *)(wad + dir->pos);
    }
  }

  for (struct vertex *v = vs; v - vs < numV; v++) {
    if (v->x < l->bounds[0].x) {
      l->bounds[0].x = v->x;
    }

    if (v->x > l->bounds[1].x) {
      l->bounds[1].x = v->x;
    }

    if (v->y < l->bounds[0].y) {
      l->bounds[0].y = v->y;
    }

    if (v->y > l->bounds[1].y) {
      l->bounds[1].y = v->y;
    }
  }

  l->lines = malloc(l->numL * sizeof(sgp_line));
  for (int i = 0; i < l->numL; i++) {
    struct line line = ls[i];
    struct vertex v1 = vs[line.v1];
    struct vertex v2 = vs[line.v2];
    l->lines[i] = (sgp_line){{v1.x, v1.y}, {v2.x, v2.y}};
  }

  printf("num lines %d\n", l->numL);
}

static void stuff(void) {
  struct header head = *(struct header *)wad;

  printf("%.4s\n", head.id);

  struct dir *dir = (struct dir *)(wad + head.ofs);
  for (int i = 0; i < head.num; i++) {
    if (i < (head.num - 1) && dir[i].size == 0 &&
        !strncmp("THINGS", dir[i + 1].name, 6))
      level(dir + i);
  }
}

static float scale = 1.0;

static void frame(void) {
  // Get current window size.
  int width = sapp_width(), height = sapp_height();
  int smallest = width < height ? width : height;
  float ratio = width / (float)height;

  // Begin recording draw commands for a frame buffer of size (width, height).
  sgp_begin(width, height);
  // Set frame buffer drawing region to (0,0,width,height).
  sgp_viewport((width - smallest) / 2, (height - smallest) / 2, smallest,
               smallest);
  // Set drawing coordinate space to (left=-ratio, right=ratio, top=1,
  // bottom=-1).
  sgp_project(-32768, 32767, -32768, 32767);

  sgp_scale(scale, scale);

  // Clear the frame buffer.
  sgp_set_color(0.1f, 0.1f, 0.1f, 1.0f);
  sgp_clear();

  sgp_set_color(1.0f, 1.0f, 1.0f, 1.0f);

  sgp_draw_lines(levels[curLevel].lines, levels[curLevel].numL);

  sg_pass pass = {.swapchain = sglue_swapchain()};
  sg_begin_pass(&pass);
  sgp_flush();
  sgp_end();
  sg_end_pass();
  sg_commit();
}

static void event(const sapp_event *evt) {
  if (evt->type == SAPP_EVENTTYPE_MOUSE_SCROLL) {
    scale += evt->scroll_y;
    printf("scroll y: %f new scale: %f\n", evt->scroll_y, scale);
  }
  if (evt->type != SAPP_EVENTTYPE_KEY_DOWN) {
    return;
  }

  switch (evt->key_code) {
  case SAPP_KEYCODE_LEFT:
  case SAPP_KEYCODE_UP:
    if (curLevel != 0)
      curLevel--;
    break;
  case SAPP_KEYCODE_RIGHT:
  case SAPP_KEYCODE_DOWN:
    if (curLevel < numLevels - 1)
      curLevel++;
    break;
  case SAPP_KEYCODE_ESCAPE:
  case SAPP_KEYCODE_Q:
    exit(0);
  default:
    break;
  }
}

static void init(void) {
  // initialize Sokol GFX
  sg_desc sgdesc = {.environment = sglue_environment(),
                    .logger.func = slog_func};
  sg_setup(&sgdesc);
  if (!sg_isvalid()) {
    fprintf(stderr, "Failed to create Sokol GFX context!\n");
    exit(-1);
  }

  // initialize Sokol GP
  sgp_desc sgpdesc = {0};
  sgp_setup(&sgpdesc);
  if (!sgp_is_valid()) {
    fprintf(stderr, "Failed to create Sokol GP context: %s\n",
            sgp_get_error_message(sgp_get_last_error()));
    exit(-1);
  }

  int fd = open("/home/me/games/doom/iwads/doom.wad", O_RDONLY);
  if (fd == -1) {
    fprintf(stderr, "failed to open wad file %d %s", errno, strerror(errno));
    exit(-1);
  }

  struct stat s;
  if (fstat(fd, &s)) {
    fprintf(stderr, "failed to stat wad file %d %s", errno, strerror(errno));
    exit(-1);
  }

  wad = mmap(0, s.st_size, PROT_READ, MAP_PRIVATE, fd, 0);
  if (wad == MAP_FAILED) {
    fprintf(stderr, "failed to mmap wad file %d %s", errno, strerror(errno));
    exit(-1);
  }

  close(fd);

  stuff();
}

static void cleanup(void) {
  sgp_shutdown();
  sg_shutdown();
}

sapp_desc sokol_main(int argc, char *argv[]) {
  (void)argc;
  (void)argv;
  return (sapp_desc){
      .init_cb = init,
      .frame_cb = frame,
      .cleanup_cb = cleanup,
      .event_cb = event,
      .window_title = "Primitives (Sokol GP)",
      .logger.func = slog_func,
      .sample_count = 4,
  };
}
