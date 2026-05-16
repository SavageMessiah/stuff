#include "raylib.h"
#include "util.h"
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

enum EventType {
  EVT_SPAWNED,
  EVT_DIED,
  EVT_BONK,
  EVT_HURT,
  EVT_COUNT,
};

struct Look {
  uint8_t tileX;
  uint8_t tileY;
  Color fg;
  Color bg;
};

struct BlockDefId {
  uint16_t id;
};

struct Id {
  uint16_t generation;
  uint16_t idx;
};

enum SoundId {
  SND_EMPTY,
  SND_BONK,
  SND_TINK,
  SND_BOOM,
  SND_COUNT,
};

struct ActionContext;

typedef void (*Action)(struct ActionContext *ctx);
typedef bool (*Filter)(struct ActionContext *ctx);

struct Actor {
  Action action;
  Filter filter;
  enum SoundId sound;
  union {
    struct BlockDefId type;
    uint16_t amount;
  };
};

struct Timer {
  uint16_t tics;
  struct Actor actor;
};

#define BLOCK_DEF_NAME_LEN 32
#define MAX_TIMERS 8
struct BlockDef {
  struct BlockDefId id;
  char name[BLOCK_DEF_NAME_LEN];
  uint16_t health;
  struct Look look;
  struct Timer timers[MAX_TIMERS];
  struct Actor events[EVT_COUNT];
};

struct Pos {
  uint8_t x;
  uint8_t y;
};

struct Block {
  struct Id id;
  struct BlockDefId type;
  struct Pos pos;
  bool alive;
  uint16_t health;
  uint16_t str;
  uint16_t energy;
  uint16_t timers[MAX_TIMERS];
};

struct ActionContext {
  struct Actor *actor;
  struct Block *block;
  struct BlockDef *def;
  struct Event *evt;
};

void ActNothing(struct ActionContext *ctx);
void ActCharge(struct ActionContext *ctx);
void ActReplace(struct ActionContext *ctx);
void ActExplode(struct ActionContext *ctx);

bool FilterTargetSelf(struct ActionContext *ctx);

struct BlockDef defs[] = {
    {
        .name = "error",
        .look = {.tileX = 0, .tileY = 0, .fg = BLANK, .bg = BLANK},
    },
    {
        .name = "brick",
        .health = 100,
        .look = {.tileX = 10, .tileY = 17, .fg = MAROON, .bg = BLACK},
    },
    {
        .name = "bone",
        .health = 50,
        .look = {.tileX = 32, .tileY = 12, .fg = WHITE, .bg = BLANK},
        .timers = {{.tics = 10, .actor = {.action = ActCharge}}},
    },
    {
        .name = "trap",
        .health = 100,
        .look =
            {
                .tileX = 10,
                .tileY = 17,
                .fg = RED,
                .bg = BLACK,
            },
        .events = {[EVT_BONK] = {.action = ActReplace,
                                 .filter = FilterTargetSelf,
                                 .sound = SND_BONK,
                                 .type = {4}}},
    },
    {
        .name = "bomb",
        .health = 10,
        .look =
            {
                .tileX = 45,
                .tileY = 9,
                .fg = BLACK,
                .bg = BLANK,
            },
        .timers =
            {
                {
                    .tics = 30,
                    .actor =
                        {
                            .action = ActExplode,
                            .sound = SND_BOOM,
                            .amount = 50,
                        },
                },
                {
                    .tics = 2,
                    .actor =
                        {
                            .action = ActNothing,
                            .sound = SND_TINK,
                        },
                },
            },
    }};

static inline size_t BlockDefCount() {
  return sizeof(defs) / sizeof(struct BlockDef);
}

void InitBlockDefs() {
  for (size_t idx = 0; idx < BlockDefCount(); idx++) {
    defs[idx].id.id = idx;
  }
}

struct BlockDef *BlockDefGet(struct BlockDefId id) {
  expect(id.id < BlockDefCount());
  return &defs[id.id];
}

struct BlockDefId BlockDefForName(const char *name) {
  for (size_t idx = 0; idx < BlockDefCount(); idx++) {
    struct BlockDef *def = &defs[idx];
    if (!strncmp(def->name, name, BLOCK_DEF_NAME_LEN)) {
      return (struct BlockDefId){idx};
    }
  }

  return (struct BlockDefId){};
}

#define W 40
#define H 20
#define T 32

Rectangle tileRect(uint8_t x, uint8_t y) {
  return (Rectangle){x * T, y * T, T, T};
}

Texture2D tileset;

void DrawBlockEx(struct Look *look, Vector2 pos, uint8_t alpha) {
  if (!ColorIsEqual(look->bg, BLANK)) {
    Color bg = look->bg;
    bg.a = alpha;
    DrawRectangleV(pos, (Vector2){.x = T, .y = T}, bg);
  }
  Color fg = look->fg;
  fg.a = alpha;
  DrawTextureRec(tileset, tileRect(look->tileX, look->tileY), pos, fg);
}

void DrawBlock(struct Look *look, Vector2 pos) { DrawBlockEx(look, pos, 255); }

void LoadTileset() {
  Image tilesetSmall = LoadImage("monochrome-transparent_packed.png");
  ImageResizeNN(&tilesetSmall, tilesetSmall.width * 2, tilesetSmall.height * 2);

  tileset = LoadTextureFromImage(tilesetSmall);

  UnloadImage(tilesetSmall);
}

struct Event {
  enum EventType type;
  struct Id source;
  struct Id target;
};

#define MAX_EVENTS 1024
struct EventBuffer {
  size_t idx;
  struct Event events[MAX_EVENTS];
};

void EventEmit(struct EventBuffer *buf, struct Event *evt) {
  expect(buf->idx < MAX_EVENTS);

  buf->events[buf->idx++] = *evt;
}

void EventReset(struct EventBuffer *buf) { buf->idx = 0; }

#define MAX_PLAYING_SOUNDS 8
Sound sounds[SND_COUNT][MAX_PLAYING_SOUNDS] = {0};

void LoadOneSound(enum SoundId id, const char *filename) {
  sounds[id][0] = LoadSound(filename);
  for (size_t i = 1; i < MAX_PLAYING_SOUNDS; i++) {
    sounds[id][i] = LoadSoundAlias(sounds[id][0]);
  }
}

void LoadSounds() {
  LoadOneSound(SND_EMPTY, "dsvilsit.wav");
  LoadOneSound(SND_BONK, "dspunch.wav");
  LoadOneSound(SND_TINK, "dstink.wav");
  LoadOneSound(SND_BOOM, "dsbarexp.wav");
}

void UnloadSounds() {
  for (size_t i = 0; i < SND_COUNT; i++) {
    for (size_t j = 1; j < MAX_PLAYING_SOUNDS; j++) {
      UnloadSoundAlias(sounds[i][j]);
    }

    UnloadSound(sounds[i][0]);
  }
}

void PlaySoundInstance(enum SoundId id) {
  Sound *this = sounds[id];
  for (size_t i = 0; i < MAX_PLAYING_SOUNDS; i++) {
    if (!IsSoundPlaying(this[i])) {
      PlaySound(this[i]);
      return;
    }
  }
}

#define MAX_BLOCKS 40 * 20

struct Id map[MAX_BLOCKS] = {};
struct Block blocks[MAX_BLOCKS] = {};
struct EventBuffer events = {};

struct SpawnResult {
  struct Id id;
  bool spawned;
  bool blocked;
};

struct Block *BlockGet(struct Id id) {
  if (id.idx >= MAX_BLOCKS) {
    return NULL;
  }

  struct Block *block = &blocks[id.idx];
  if (block->id.generation != id.generation || !block->alive) {
    return NULL;
  }

  return block;
}

struct SpawnResult BlockSpawn(struct Block new) {
  struct Id *loc = &map[new.pos.x + new.pos.y * W];
  struct Block *old = BlockGet(*loc);
  if (old) {
    return (struct SpawnResult){.blocked = true};
  }

  // reserve the first block to always be the empty block
  for (struct Block *block = blocks; block < blocks + MAX_BLOCKS; block++) {
    if (block->alive) {
      continue;
    }
    new.id.generation = block->id.generation + 1;
    new.id.idx = block - blocks;
    new.alive = true;

    struct BlockDef *def = BlockDefGet(new.type);
    for (size_t idx = 0; idx < MAX_TIMERS; idx++) {
      new.timers[idx] = def->timers[idx].tics;
    }

    *block = new;

    struct SpawnResult res = {
        .id = block->id,
        .spawned = true,
    };

    EventEmit(&events, &(struct Event){
                           .type = EVT_SPAWNED,
                           .target = res.id,
                       });

    *loc = res.id;

    return res;
  }

  expect("ran out of blocks");
  __builtin_unreachable();
}

bool BlockKill(struct Id id, struct Id source) {
  struct Block *block = BlockGet(id);
  if (!block) {
    return false;
  }

  block->alive = false;
  EventEmit(&events,
            &(struct Event){.type = EVT_DIED, .target = id, .source = source});

  return true;
}

bool BlockHurt(struct Id id, uint16_t amount, struct Id source) {
  struct Block *block = BlockGet(id);
  if (!block) {
    return false;
  }

  if (block->health > amount) {
    block->health -= amount;
    EventEmit(&events, &(struct Event){
                           .type = EVT_HURT, .source = source, .target = id});
    return false;
  }

  block->health = 0;
  BlockKill(block->id, source);

  return true;
}

void BlockNeighborsGet(struct Id id, struct Id *buf) {
  struct Block *block = BlockGet(id);
  if (!block->alive) {
    return;
  }
  struct Pos pos = block->pos;

  if (pos.x > 0) {
    buf[0] = map[pos.x - 1 + pos.y * W];
  }
  if (pos.y > 0) {
    buf[1] = map[pos.x + (pos.y - 1) * W];
  }
  if (pos.y + 1 < H) {
    buf[2] = map[pos.x + (pos.y + 1) * W];
  }
  if (pos.x + 1 < W) {
    buf[3] = map[pos.x + 1 + pos.y * W];
  }
}

void InitMap() {
  struct BlockDefId brick = BlockDefForName("brick");
  for (uint8_t i = 0; i < W; i++) {
    for (uint8_t j = 18; j < H; j++) {
      BlockSpawn((struct Block){.type = brick, .pos = {.x = i, .y = j}});
    }
  }

  struct BlockDefId bone = BlockDefForName("bone");
  BlockSpawn((struct Block){.type = bone, .pos = {.x = 30, .y = 12}});
}

void RunActor(struct ActionContext *ctx) {
  if (!ctx->actor->action || (ctx->actor->filter && !ctx->actor->filter(ctx))) {
    return;
  }

  if (ctx->actor->sound != SND_EMPTY) {
    PlaySoundInstance(ctx->actor->sound);
  }

  ctx->actor->action(ctx);
}

struct Ticker {
  uint32_t tics;
  double speed;
  double accum;
  bool running;
};

void TickerInit(struct Ticker *t) {
  memset(t, 0, sizeof(struct Ticker));
  t->speed = 0.1;
}

void TickerAddTime(struct Ticker *t, double time) {
  if (t->running) {
    t->accum += time;
  }
}

uint32_t TickerTick(struct Ticker *t) {
  if (t->accum < t->speed) {
    return 0;
  }

  t->accum -= t->speed;
  return ++t->tics;
}

struct Ticker ticker;

void RunTimers() {
  for (struct Block *block = blocks; block < blocks + MAX_BLOCKS; block++) {
    if (!block->alive) {
      continue;
    }
    struct BlockDef *def = BlockDefGet(block->type);
    struct ActionContext ctx = {
        .block = block,
        .def = def,
    };
    for (size_t idx = 0; idx < MAX_TIMERS; idx++) {
      struct Timer *timer = &def->timers[idx];
      uint16_t *time = &block->timers[idx];
      if (timer->tics == 0) {
        break;
      }
      (*time)--;
      if (*time != 0) {
        continue;
      }

      ctx.actor = &timer->actor;
      RunActor(&ctx);
      *time = timer->tics;
    }
  }
}

void ActNothing(struct ActionContext *ctx) {}

void ActCharge(struct ActionContext *ctx) {
  struct Pos *pos = &ctx->block->pos;
  if (pos->x == W - 6) {
    // kill the block
    BlockKill(ctx->block->id, (struct Id){0});
    return;
  }
  uint8_t newX = pos->x + 1;
  size_t idx = pos->x + pos->y * W;
  size_t newIdx = newX + pos->y * W;
  struct Block *front = BlockGet(map[newIdx]);
  if (front) {
    EventEmit(&events, &(struct Event){.type = EVT_BONK,
                                       .source = ctx->block->id,
                                       .target = front->id});
    return;
  }

  pos->x = newX;
  map[newIdx] = map[idx];
  map[idx] = (struct Id){0};
}

void ActReplace(struct ActionContext *ctx) {
  struct Pos pos = ctx->block->pos;
  BlockKill(ctx->block->id, ctx->block->id);
  BlockSpawn((struct Block){.type = ctx->actor->type, .pos = pos});
}

void ActExplode(struct ActionContext *ctx) {
  printf("LMAO!\n");

  struct Id neighbors[4] = {0};
  BlockNeighborsGet(ctx->block->id, neighbors);

  for (size_t idx = 0; idx < 4; idx++) {
    struct Block *block = BlockGet(neighbors[idx]);
    if (!block) {
      continue;
    }

    BlockHurt(neighbors[idx], ctx->actor->amount, ctx->block->id);
  }

  BlockKill(ctx->block->id, ctx->block->id);
}

bool FilterTargetSelf(struct ActionContext *ctx) {
  return !memcmp(&ctx->block->id, &ctx->evt->target, sizeof(struct Id));
}

int main(void) {
  InitWindow(W * T, 960, "moose");

  LoadTileset();
  InitAudioDevice();
  LoadSounds();
  InitBlockDefs();
  InitMap();
  // toss events from map creation
  EventReset(&events);
  TickerInit(&ticker);

  SetTargetFPS(60);

  struct BlockDef *selectedBlock = NULL;
  uint32_t frame = 0;

  while (!WindowShouldClose()) {
    frame++;
    Vector2 mouse = GetMousePosition();
    bool clicked = IsMouseButtonPressed(0);

    if (IsKeyPressed(KEY_SPACE)) {
      if (ticker.running) {
        ticker.running = false;
      } else {
        printf("running\n");
        ticker.running = true;
      }
    }

    TickerAddTime(&ticker, GetFrameTime());
    while (TickerTick(&ticker)) {
      RunTimers();
      if (events.idx > 0) {
        printf("events! %zu\n", events.idx);
      }
      for (struct Event *evt = events.events; evt < events.events + events.idx;
           evt++) {
        for (struct Block *block = blocks; block < blocks + MAX_BLOCKS;
             block++) {
          if (!block->alive) {
            continue;
          }

          struct BlockDef *def = BlockDefGet(block->type);

          struct Actor *actor = &def->events[evt->type];
          struct ActionContext ctx = {
              .block = block,
              .def = def,
              .actor = actor,
              .evt = evt,
          };
          RunActor(&ctx);
        }

        switch (evt->type) {
        case EVT_SPAWNED:
          printf("spawned %d\n", evt->target.idx);
          break;
        case EVT_DIED:
          printf("%d killed %d\n", evt->source.idx, evt->target.idx);
          break;
        case EVT_BONK:
          printf("%d bonked %d\n", evt->source.idx, evt->target.idx);
          break;
        case EVT_HURT:
          printf("%d hurt %d\n", evt->source.idx, evt->target.idx);
          break;
        default:
          printf("sumthin\n");
        };
      }
      EventReset(&events);
    }

    BeginDrawing();
    ClearBackground(BLACK);

    DrawRectangle(0, 0, 40 * T, 20 * T, SKYBLUE);

    for (uint8_t i = 0; i < W; i++) {
      for (uint8_t j = 0; j < H; j++) {
        Vector2 pos = {.x = i * T, .y = j * T};
        struct Id id = map[i + j * W];
        struct Block *block = BlockGet(id);
        if (block) {
          DrawBlock(&BlockDefGet(block->type)->look, pos);
        }
        Rectangle rec = {.x = pos.x, .y = pos.y, .width = T, .height = T};
        if (CheckCollisionPointRec(mouse, rec)) {
          if (selectedBlock) {
            DrawBlockEx(&selectedBlock->look, pos, 100);
            if (block) {
              DrawRectangleRec(rec, (Color){.r = 255, .a = 100});
            }
            if (clicked && !block) {
              BlockSpawn((struct Block){.type = selectedBlock->id,
                                        .pos = {.x = i, .y = j}});
              selectedBlock = NULL;
            }
          }
        }
      }
    }

    DrawText("I do a thing", 190, 700, 20, LIGHTGRAY);
    char framebuf[32] = {0};
    snprintf(framebuf, 31, "%d", ticker.tics);
    DrawText(framebuf, 190, 650, 20, LIGHTGRAY);
    Rectangle rec = {.x = 10, .y = 650, .width = T, .height = T};
    Color timerColor = WHITE;
    if (CheckCollisionPointRec(mouse, rec)) {
      timerColor = BLUE;
      if (clicked) {
        ticker.running = !ticker.running;
      }
    }
    int offset = ticker.running ? frame / 20 % 3 : 0;
    DrawTextureRec(tileset, tileRect(40 + offset, 12),
                   (Vector2){.x = rec.x, .y = rec.y}, timerColor);

    for (int i = 1; i < 4; i++) {
      Rectangle rec = {
          .x = 10 + (T + 8) * i, .y = 650, .width = T, .height = T};
      Color speedColor = WHITE;
      if (CheckCollisionPointRec(mouse, rec)) {
        speedColor = BLUE;
        if (clicked) {
          ticker.speed = 0.1 / i;
        }
      }
      if (ticker.speed == 0.1 / i) {
        speedColor = GREEN;
      }
      DrawTextureRec(tileset, tileRect(35 + i, 17),
                     (Vector2){.x = rec.x, .y = rec.y}, speedColor);
    }

    for (size_t idx = 0; idx < BlockDefCount(); idx++) {
      struct BlockDef *def = BlockDefGet((struct BlockDefId){idx});
      Vector2 pos = {.x = 100 + idx * T, .y = 750};
      DrawBlock(&def->look, pos);
      Rectangle rec = {.x = pos.x, .y = pos.y, .width = T, .height = T};
      if (CheckCollisionPointRec(mouse, rec)) {
        DrawRectangleRec(rec, (Color){.r = 100, .g = 200, .b = 0, .a = 100});
        if (clicked) {
          selectedBlock = def;
          printf("picked %s\n", def->name);
        }
      } else {
        if (selectedBlock == def) {
          DrawRectangleRec(rec, (Color){.r = 0, .g = 100, .b = 200, .a = 100});
        }
      }
    }

    EndDrawing();
  }

  CloseWindow();

  UnloadSounds();
  CloseAudioDevice();
  UnloadTexture(tileset);

  return 0;
}
