#pragma once
#include <string>
#include <SFML/System.hpp>
#include <SFML/Graphics.hpp>
// Needed for call to abs() in tile movement algorithm
#include <cmath>

#include "animatedsprite.hpp"
#include "debug.hpp"

enum Direction
{
    UP,
    DOWN,
    LEFT,
    RIGHT
};

class Character : public sf::Drawable
{
public:

    Character(sf::String, sf::String, sf::Vector2i);
    ~Character();
    // Update the character's position, et cetera based on a delta-time
    void Update(sf::Time);
    
    // Teleport to a tile on the current map.
    void SetPosition(sf::Vector2i);
    // Get the current tile on which the Character is standing
    sf::Vector2i GetPosition();
    sf::Vector2f GetPixelPosition();
    
    void SetID(sf::String);
    std::wstring GetID();
    
    // Load an animated 4-direction sprite from a file
    void LoadSprite(sf::String);
    
    void Move(Direction);
    
    virtual void draw(sf::RenderTarget&, sf::RenderStates) const;

private:
    sf::String m_ID;
    sf::String m_Name;
    // Tile position - current and target
    sf::Vector2i m_TilePositionNow;
    sf::Vector2i m_TilePositionNext = sf::Vector2i(m_TilePositionNow);
    float m_Speed = 2; // Speed in tiles/sec
    
    // Pixel-perfect position for drawing
    sf::Vector2f m_PixelPosition;
    
    // Locking, to prevent changing target while moving
    bool m_Locked;
    
    // The graphical representation of the Character
    sf::Texture *m_Spritesheet;
    Animation m_WalkingAnimationUp, m_WalkingAnimationDown, m_WalkingAnimationLeft, m_WalkingAnimationRight;
    Animation *m_CurrentAnimation;
    AnimatedSprite m_Sprite;
};


//int _lua_character_load(lua_State *L);