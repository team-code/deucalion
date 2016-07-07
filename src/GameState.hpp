#pragma once
#include <iostream>
#include <unordered_map>
#include <string>
#include <SFML/System.hpp>
#include <SFML/Graphics.hpp>
#include <STP/TMXLoader.hpp>

#include "Character.hpp"
#include "debug.hpp"

class GameState : public sf::Drawable
{
public:
    GameState();
    ~GameState();
    // Update the game's state for a delta time
    void Update(sf::Time);
    
    void LoadMap(sf::String);
    
    void AddCharacter(Character);
    
    // Set a character (by ID) as being controlled by the player rather than by AI
    void SetPlayer(sf::String);
    
    // Tell the player object that its being asked to move 
    void MovePlayer(Direction);
    
    // Return the pixel position of the player (for focusing the camera)
    sf::Vector2f GetPlayerPixelPosition();
    
    virtual void draw(sf::RenderTarget&, sf::RenderStates) const;

private:
    // These hide the upper and lower layers of the current map, for rendering
    void ShowLowerLayers() const;
    void ShowUpperLayers() const;
    
    sf::String m_CurrentMapPath;
    tmx::TileMap* m_CurrentMap;
    std::unordered_map<std::wstring, Character> m_Characters;
    // The character being controlled by the player
    Character *m_Player;

};

