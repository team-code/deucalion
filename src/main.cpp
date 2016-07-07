#include <iostream>
using namespace std;

#include <SFML/Graphics.hpp>

#include "GameState.hpp"
#include "scripting.hpp"
#include "engine_config.hpp"

int main()
{
    // Init Lua so we can execute scripts
    lua_State* L = scripting_init();
    
    // Get the engine config
    struct engine_config_s engine_config = get_engine_config(L, "./data/engine_config.lua");
    
    GameState* game_state = new GameState();
    
    sf::RenderWindow window(sf::VideoMode(engine_config.screen_width, engine_config.screen_height), "Game Title");
    window.setFramerateLimit(engine_config.maximum_framerate);
    
    // Set up the view
    sf::View main_view;
    // TODO: No more magic numbers! (configurable screen resolution)
    main_view.setCenter(640/2, 480/2);
    main_view.setSize(640, 480);
    window.setView(main_view);
    
    game_state->AddCharacter(Character("Test", "data/characters/player/player.png", sf::Vector2i(5, 10)));
    
    game_state->SetPlayer("Test");
    
    game_state->LoadMap("data/maps/map001/map001.tmx");
    
    sf::Event* event = new sf::Event;
    
    
    sf::Clock* frame_clock = new sf::Clock;
    while (window.isOpen()){
        // Check events and react accordingly
        while (window.pollEvent(*event)){
            if (event->type == sf::Event::Closed) {
                window.close();
            } 
        }
        
        // Move the player based on key presses
        // TODO: Make this configurable
        // TODO: Only do this if the window has focus
        if (sf::Keyboard::isKeyPressed(sf::Keyboard::Left)) {
            game_state->MovePlayer(Direction::LEFT);
        } else if (sf::Keyboard::isKeyPressed(sf::Keyboard::Right)) {
            game_state->MovePlayer(Direction::RIGHT);
        } else if (sf::Keyboard::isKeyPressed(sf::Keyboard::Up)) {
            game_state->MovePlayer(Direction::UP);
        } else if (sf::Keyboard::isKeyPressed(sf::Keyboard::Down)) {
            game_state->MovePlayer(Direction::DOWN);
        }
        
        // Update the state (.restart() returns the current time)
        game_state->Update(frame_clock->restart());
        
              
        // Draw the world, now that we have the new state
        main_view.setCenter(game_state->GetPlayerPixelPosition());
        window.clear();
        window.draw(*game_state);
        
        // Draw the FX layer, once it is implemented
        //  (things like rain, black/redout, et cetera)
        
        // Draw the user interface; this ordering prevents it from 
        //  being affected by the FX layer
        
        // Final drawing step
        window.setView(main_view);
        window.display();
    }
    
    delete event;
    return 0;
}