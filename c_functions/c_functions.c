#include "c_functions.h"
#include <SDL2/SDL.h>


static uint16_t _paddle_length;
static uint8_t _paddle_width;
static uint8_t _ball_size;
static int _width, _height;
static SDL_Rect _rect;

static SDL_Event window_event;
static SDL_Window *win;
static SDL_Renderer * renderer;

bool game_over = false;

bool p1_up   = false;
bool p1_down = false;
bool p2_up   = false;
bool p2_down = false;

void draw(float ball_x, float ball_y, float p1_height, float p2_height)
{
    SDL_RenderClear(renderer);
    SDL_SetRenderDrawColor(renderer, 0, 0, 0, 0);
    
    SDL_RenderFillRect(renderer, &_rect);
    SDL_SetRenderDrawColor(renderer, 255, 255, 255, 0);
    SDL_Rect b = {
        .h = _ball_size,
        .w = _ball_size,
        .x = (int)ball_x - (int)(_ball_size*0.5),
        .y = (int)ball_y - (int)(_ball_size*0.5)
    };

    SDL_RenderFillRect(renderer, &b);

    SDL_Rect p1 = {
        .h = _paddle_length,
        .w = _paddle_width,
        .x = 0,
        .y = (int)p1_height
    };
    SDL_RenderFillRect(renderer, &p1);
    SDL_Rect p2 = {
        .h = _paddle_length,
        .w = _paddle_width,
        .x = _width - _paddle_width,
        .y = (int)p2_height
    };
    SDL_RenderFillRect(renderer, &p2);
    SDL_RenderPresent(renderer);
}


void init(int width, int height, uint16_t paddle_length, uint8_t paddle_width, uint8_t ball_size) {
    _paddle_length = paddle_length;
    _paddle_width = paddle_width;
    _ball_size = ball_size;
    _width = width;
    _height = height;
    _rect = (SDL_Rect){.h = _height, .w = _width};

    SDL_Init(SDL_INIT_EVERYTHING);
    SDL_CreateWindowAndRenderer(width, height, 0, &win, &renderer);

    SDL_RenderSetScale(renderer, 1, 1);
}

uint32_t get_time_milis() {
    return SDL_GetTicks();
}

void finish_game() {
    game_over = true;
    SDL_DestroyWindow(win);
    SDL_Quit();
}

void set_title(char * title) {
    SDL_SetWindowTitle(win, title);
}

void update_SDL(){
    if (SDL_PollEvent(&window_event))
    {
        switch (window_event.type)
        {
        case SDL_QUIT:
            finish_game();
            break;
        case SDL_KEYDOWN:
            switch (window_event.key.keysym.scancode)
            {
            case SDL_SCANCODE_UP:
                p2_up = true;
                break;
            case SDL_SCANCODE_DOWN:
                p2_down = true;
                break;
            case SDL_SCANCODE_W:
                p1_up = true;
                break;
            case SDL_SCANCODE_S:
                p1_down = true;
                break;
            default:
                break;
            }
            break;
        case SDL_KEYUP:
            switch (window_event.key.keysym.scancode)
            {
            case SDL_SCANCODE_UP:
                p2_up = false;
                break;
            case SDL_SCANCODE_DOWN:
                p2_down = false;
                break;
            case SDL_SCANCODE_W:
                p1_up = false;
                break;
            case SDL_SCANCODE_S:
                p1_down = false;
                break;
            default:
                break;
            }
            break;
        default:
            break;
        }
    }
}