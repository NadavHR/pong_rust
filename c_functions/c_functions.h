#ifndef C_FUNCTIONS
#define C_FUNCTIONS
#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>

void init(int width, int height, uint16_t paddle_length, uint8_t paddle_width, uint8_t ball_size);
uint32_t get_time_milis();
void draw(float ball_x, float ball_y, float p1_height, float p2_height);
void update_SDL();
void finish_game();
void set_title(char * title);
bool p1_up, p1_down, p2_up, p2_down, game_over;
#endif