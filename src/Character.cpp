#include "Character.hpp"

Character::Character(sf::String ID, sf::String animation_path, sf::Vector2i position)
{
    m_ID = ID;
    LoadSprite(animation_path);
    SetPosition(position);    
}

Character::~Character()
{
}

void Character::SetPosition(sf::Vector2i new_position)
{
    m_TilePositionNow = new_position;
    m_TilePositionNext = new_position;
}

sf::Vector2i Character::GetPosition()
{
    return m_TilePositionNow;
}

sf::Vector2f Character::GetPixelPosition()
{
    return m_PixelPosition;
}

std::wstring Character::GetID()
{
    return m_ID.toWideString();
}

void Character::LoadSprite(sf::String path)
{
    m_Spritesheet = new sf::Texture;
    if(!m_Spritesheet->loadFromFile(path)){
        debug("Failed to load spritesheet from %s", path.toAnsiString().c_str());
    } else {
        debug("Loaded a spritesheet from %s", path.toAnsiString().c_str());
    }
    
    // Attach each animation - center/right/center/left, incrementing the Y value
    //  for each row.
    m_WalkingAnimationDown.setSpriteSheet(*m_Spritesheet);
    m_WalkingAnimationDown.addFrame(sf::IntRect(32, 0, 32, 32));
    m_WalkingAnimationDown.addFrame(sf::IntRect(64, 0, 32, 32));
    m_WalkingAnimationDown.addFrame(sf::IntRect(32, 0, 32, 32));
    m_WalkingAnimationDown.addFrame(sf::IntRect( 0, 0, 32, 32));

    m_WalkingAnimationLeft.setSpriteSheet(*m_Spritesheet);
    m_WalkingAnimationLeft.addFrame(sf::IntRect(32, 32, 32, 32));
    m_WalkingAnimationLeft.addFrame(sf::IntRect(64, 32, 32, 32));
    m_WalkingAnimationLeft.addFrame(sf::IntRect(32, 32, 32, 32));
    m_WalkingAnimationLeft.addFrame(sf::IntRect( 0, 32, 32, 32));

    m_WalkingAnimationRight.setSpriteSheet(*m_Spritesheet);
    m_WalkingAnimationRight.addFrame(sf::IntRect(32, 64, 32, 32));
    m_WalkingAnimationRight.addFrame(sf::IntRect(64, 64, 32, 32));
    m_WalkingAnimationRight.addFrame(sf::IntRect(32, 64, 32, 32));
    m_WalkingAnimationRight.addFrame(sf::IntRect( 0, 64, 32, 32));

    m_WalkingAnimationUp.setSpriteSheet(*m_Spritesheet);
    m_WalkingAnimationUp.addFrame(sf::IntRect(32, 96, 32, 32));
    m_WalkingAnimationUp.addFrame(sf::IntRect(64, 96, 32, 32));
    m_WalkingAnimationUp.addFrame(sf::IntRect(32, 96, 32, 32));
    m_WalkingAnimationUp.addFrame(sf::IntRect( 0, 96, 32, 32));

    m_Sprite = AnimatedSprite();

    m_CurrentAnimation = &m_WalkingAnimationDown;
    m_Sprite.play(*m_CurrentAnimation);
}

void Character::Move(Direction dir)
{
    if (m_Locked) {
        // If we're locked, don't do anything.
        return;
    }

    // TODO: Check collisions somehow
    // TODO: Implement turning requiring a lock and taking a certain amount of time
    if (dir == Direction::DOWN) {
        m_TilePositionNext.y = m_TilePositionNow.y + 1;
        m_CurrentAnimation = &m_WalkingAnimationDown;
    } else if (dir == Direction::UP) {
        m_TilePositionNext.y = m_TilePositionNow.y - 1;
        m_CurrentAnimation = &m_WalkingAnimationUp;
    } else if (dir == Direction::LEFT) {
        m_TilePositionNext.x = m_TilePositionNow.x - 1;
        m_CurrentAnimation = &m_WalkingAnimationLeft;
    } else if (dir == Direction::RIGHT) {
        m_TilePositionNext.x = m_TilePositionNow.x + 1;
        m_CurrentAnimation = &m_WalkingAnimationRight;
    }
    //debug("Tile now: %d %d Tile next: %d %d", m_TilePositionNow.x, m_TilePositionNow.y, 
    //                                          m_TilePositionNext.x, m_TilePositionNext.y);
}

void Character::Update(sf::Time delta_time)
{
    if (m_TilePositionNow == m_TilePositionNext){
        // If we're on the tile we need to be on, meaning we're not moving
        // TODO: Change the magic 32 to be configurable once we sort out configuration
        m_PixelPosition = sf::Vector2f(m_TilePositionNow * 32);
        m_Sprite.pause();
        m_Locked = false;
    } else {
        m_Locked = true;
        // We're currently moving to another tile.
        // Are we there yet? We check if we're within a pixel of the destination; if so, we're no longer moving.
        // TODO: Again, magic numbers here (for the (1, 1) pixel magic snapping limit), but this is less of a priority
        sf::Vector2f temp_dist_vector = m_PixelPosition - sf::Vector2f(m_TilePositionNext * 32);
        //debug("Temp. Distance Vector: %f %f ", temp_dist_vector.x, temp_dist_vector.y);
        if ( std::abs(temp_dist_vector.x) <= 1 && std::abs(temp_dist_vector.y) <= 1 ) {
            // If we're there, set things up so that next time we don't need to move
            m_TilePositionNow = m_TilePositionNext;
            // TODO: Do something so that we can respond to a move request immediately, without an intervening frame of stasis.
        } else {
            // Play our animation
            m_Sprite.play(*m_CurrentAnimation);
            // and move toward the new tile based on our speed
            // TODO: Again, make the magic 32 here based on configuration
            m_PixelPosition.x += (m_TilePositionNext.x - m_TilePositionNow.x) * (m_Speed * 32) * delta_time.asSeconds();
            m_PixelPosition.y += (m_TilePositionNext.y - m_TilePositionNow.y) * (m_Speed * 32) * delta_time.asSeconds();
            //debug("PixPos: %f %f", m_PixelPosition.x, m_PixelPosition.y);
        }
    }
    
    m_Sprite.setPosition(m_PixelPosition);
    
    if (m_Sprite.isPlaying()) {
        m_Sprite.update(delta_time);
    }
}

void Character::draw(sf::RenderTarget& target, sf::RenderStates states) const
{
    target.draw(m_Sprite);
    //debug("Drawing a sprite at %d %d (pixel %f %f)", m_TilePositionNow.x, m_TilePositionNow.y,
    //                                                    m_PixelPosition.x, m_PixelPosition.y);
}

//int _lua_character_load(lua_State *L)
//{
    
//}

