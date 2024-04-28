#include <SDL2/SDL.h>
#include <SDL2/SDL_events.h>
#include <SDL2/SDL_image.h>
#include <SDL2/SDL_render.h>
#include <SDL2/SDL_timer.h>
#include <SDL2/SDL_video.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define TILE_DIM 32
#define TILES_W 20
#define TILES_H 10
#define SNEK_MAX 200

enum GameState {
  START,
  PLAY,
  LOSE,
  WIN,
  QUIT
};

static SDL_Renderer *renderer;
static SDL_Window *window;

static SDL_Texture *snekHead;
static SDL_Texture *snekBody;
static SDL_Texture *snekAss;
static SDL_Texture *shittyFruit;

struct pos {
  int x, y;
};

static int snekFacing = 0;
static int nextSnekFacing = 0;

static struct pos fruit;
static struct pos snek[SNEK_MAX];
static unsigned int snekLen;

static enum GameState state = START;

SDL_Texture *loadTexture(char *filename) {
  SDL_Texture *tex = IMG_LoadTexture(renderer, filename);
  if(tex == NULL) {
    printf("Failed to load texture: %s %s\n", filename, SDL_GetError());
    exit(1);
  }
  return tex;
}

void draw(SDL_Texture *tex, struct pos p, int angle) {
  SDL_Rect dest;

  dest.x = p.x * TILE_DIM;
  dest.y = p.y * TILE_DIM;
  SDL_QueryTexture(tex, NULL, NULL, &dest.w, &dest.h);

  SDL_RenderCopyEx(renderer, tex, NULL, &dest, angle, NULL, SDL_FLIP_NONE);
}

int snekAvoided(struct pos p) {
  for(int i = 0; i < snekLen; i++) {
    if(snek[i].x == p.x && snek[i].y == p.y)
      return 0;
  }

  return 1;
}

void placeFruit() {
  for(int tries = 0; tries < 100; tries++) {
    struct pos maybe = {
      .x = rand() % TILES_W,
      .y = rand() % TILES_H
    };

    if( snekAvoided(maybe) ) {
      fruit = maybe;
      return;
    }
  }
  
  puts("somehow didn't manage to place a fruit in 100 tries");
  exit(1);
}

void initState() {
  snekLen = 3;
  snek[0].x = 6;
  snek[0].y = 2;
  snek[1].x = 6;
  snek[1].y = 3;
  snek[1].x = 6;
  snek[1].y = 4;

  //snek[0].x = rand() % TILES_W;
  //snek[0].y = rand() % (TILES_H - 1);
  //snek[1].x = snek[0].x;
  //snek[1].y = snek[0].y + 1;

  placeFruit();
}

void drawScene() {
  switch(state) {
    case START: 
      SDL_SetRenderDrawColor(renderer, 0, 255, 0, 255);
      SDL_RenderClear(renderer);
      break;
    case PLAY:
      SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255);
      SDL_RenderClear(renderer);

      draw(shittyFruit, fruit, 0);

      draw(snekHead, snek[0], snekFacing);

      for(int i = 1; i <= snekLen - 2; i++) {
        draw(snekBody, snek[i], 0);
      }

      int assFacing = 0;
      struct pos *ass = &snek[snekLen - 1];
      struct pos *next = &snek[snekLen - 2];
      if(next->y > ass->y) {
        assFacing = 180;
      } else if(next->x < ass->x) {
        assFacing = 270;
      } else if(next->x > ass->x) {
        assFacing = 90; 
      }

      draw(snekAss, *ass, assFacing);
      break;
    case LOSE:
      SDL_SetRenderDrawColor(renderer, 255, 0, 0, 255);
      SDL_RenderClear(renderer);
      break;
    case WIN:
      SDL_SetRenderDrawColor(renderer, 0, 0, 255, 255);
      SDL_RenderClear(renderer);
      break;
  }

  SDL_RenderPresent(renderer);
}

void handleKey(SDL_Keycode key) {
  if(key == SDLK_q) {
    state = QUIT;
    return;
  }

  switch(state) {
    case START:
      state = PLAY;
      break;
    case PLAY:
      switch(key) {
        case SDLK_w:
          nextSnekFacing = 0;
          break;
        case SDLK_a:
          nextSnekFacing = 270;
          break;
        case SDLK_s:
          nextSnekFacing = 180;
          break;
        case SDLK_d:
          nextSnekFacing = 90;
          break;
      }
      break;
    case LOSE:
      //todo: do
      break;
    default:
      printf("unknown game state: %d\n", state);
      exit(1);
  }
}

void doTick() {
  if(state != PLAY) {
    return;
  }

  snekFacing = nextSnekFacing;

  struct pos head = snek[0];

  switch(snekFacing) {
    case 0:
      head.y -= 1;
      break;
    case 90:
      head.x += 1;
      break;
    case 180:
      head.y += 1;
      break;
    case 270:
      head.x -= 1;
      break;
    default:
      printf("bad facing: %d\n", snekFacing);
      exit(1);
  }

  int grow = head.x == fruit.x && head.y == fruit.y;

  if(grow) {
    if(snekLen == SNEK_MAX) {
      printf("win!\n");
      state = WIN;
      return;
    }

    snekLen++;
  }

  for(int i = snekLen - 1; i > 0; i--) {
    snek[i] = snek[i - 1];
  }

  snek[0] = head;

  if(head.x < 0 || head.x >= TILES_W || head.y < 0 || head.y >= TILES_H) {
    printf("ded from wall\n");
    state = LOSE;
  }

  for(int i = 1; i < snekLen; i++) {
    if(head.x == snek[i].x && head.y == snek[i].y) {
      printf("ded from self eatage\n");
      state = LOSE;
      break;
    }
  }

  if(grow) {
    placeFruit();
  }
}

int main(int argc, char *argv[])
{
  srand(time(0));
  memset(&snek, 0, sizeof(snek));
  
  if(SDL_Init(SDL_INIT_VIDEO) < 0) {
    printf("Couldn't initialize SDL: %s\n", SDL_GetError());
    return 1;
  }

  if(IMG_Init(IMG_INIT_PNG) < 0) {
    printf("Couldn't intilialize SDL Image: %s\n", SDL_GetError());
    return 1;
  }

  window = SDL_CreateWindow("Snek", SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, 640, 360, 0);

  if(!window) {
    printf("Failed to open window: %s\n", SDL_GetError());
    return 1;
  }

  SDL_SetHint(SDL_HINT_RENDER_SCALE_QUALITY, "linear");

  renderer = SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED);

  if(!renderer) {
    printf("Failed to create renderer: %s\n", SDL_GetError());
    return 1;
  }

  snekHead = loadTexture("snek_head.png");
  snekBody = loadTexture("snek_body.png");
  snekAss = loadTexture("snek_ass.png");
  shittyFruit = loadTexture("shitty_fruit.png");

  initState();
  uint32_t lastTick = 0;
  while(state != QUIT) {
    uint32_t now = SDL_GetTicks();
    SDL_Event event;
    uint32_t tickDelta = now - lastTick;
    
    while(SDL_PollEvent(&event)) {
      switch(event.type) {
        case SDL_QUIT:
          return 0;
        case SDL_KEYDOWN:
          handleKey(event.key.keysym.sym);
          break;
        default:
          break;
      }
    }

    if(tickDelta > 250) {
      //printf("tick: %d\n", tickDelta);
      doTick();
      lastTick = now;
    }

    drawScene();
  }

  return 0;
}
