#include <raylib.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#define expect(x)                                                              \
  do {                                                                         \
    if (!(x)) {                                                                \
      fprintf(stderr, "Fatal error: %s:%d: assertion '%s' failed\n", __FILE__, \
              __LINE__, #x);                                                   \
      abort();                                                                 \
    }                                                                          \
  } while (0)

typedef uint16_t Type;
typedef uint16_t Id;

struct Pos {
  uint8_t x;
  uint8_t y;
};

enum BlockFlag {
  BF_ALIVE = 1,
  BF_HAS_EVENTS = 2,
};

struct Block {
  Type t;
  struct Pos pos;
  uint8_t flags;
  uint8_t health;
  uint8_t str;
  uint8_t energy;
};

struct Look {
  uint8_t tileX;
  uint8_t tileY;
  Color fg;
  Color bg;
};

struct BaseStats {
  uint8_t maxHealth;
  uint8_t armor;
};

enum EventType {
  EVT_NONE,
  EVT_TIMER,
  EVT_HURT,
  EVT_COUNT,
};

typedef uint8_t EventType;

struct BaseEvent {
  EventType type;
  uint8_t size;
};

struct HurtEvent {
  struct BaseEvent event;
  uint8_t amt;
  Id cause;
};

union EventData {
  EventType type;
  struct BaseEvent base;
  struct HurtEvent hurt;
};

struct HandlerDef {
  EventType type;
  uint32_t param;
  uint8_t flags;
  uint8_t scriptStart;
};

struct BlockDef {
  char name[32];
  struct Look look;
  struct BaseStats stats;
  struct HandlerDef handlers[8];
  uint8_t scripts[256];
};

enum Instr {
  I_NOP,
  I_CONST,
  I_ADD,
};

struct VM {
  uint32_t dataStack[256];
  uint8_t dataTop;
  uint8_t pc;
};

struct Timer {
  uint32_t t;
  Id block;
  uint8_t slot;
  uint16_t next;
};

struct BlockDef *blockDefs; // 0 reserved for empty

struct Timer timers[1024];
uint16_t firstFreeTimer;
struct Block blocks[40 * 20]; // 0 reserved for empty
Id map[40 * 20];
Id blockFree[sizeof(blocks) / sizeof(struct Block)];

#define EVT_BUFFER_SIZE 1024
struct EventSystem {
  uint16_t idx;
  uint8_t data[EVT_BUFFER_SIZE];
};

union EventData *EvtEmit(struct EventSystem *sys, uint8_t type, uint8_t size) {
  union EventData *evt = (union EventData *)(sys->data + sys->idx);
  expect(sys->idx + size < EVT_BUFFER_SIZE);
  evt->base.type = type;
  evt->base.size = size;
  sys->idx += size;

  return evt;
}

bool EvtNext(struct EventSystem *sys, union EventData **evt) {
  if (*evt) {
    // if the event pointer is non-null we've already looked at an event,
    // advance the pointer by the size
    *evt = (union EventData *)(((uint8_t *)*evt) + (*evt)->base.size);
  } else {
    // otherwise start at the beginning
    *evt = (union EventData *)sys->data;
  }

  // if the new pointer is pointing past the end, there are no more events and
  // we are done
  return ((uint8_t *)*evt) != sys->data + sys->idx;
}
