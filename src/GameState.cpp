#include "GameState.hpp"

GameState::GameState()
{
    m_CurrentMap = NULL;
}

GameState::~GameState()
{
}

void GameState::Update(sf::Time delta_time)
{
    //debug("Frame time: %f", delta_time.asSeconds());
    // Iterate through Characters, updating each
    for (std::unordered_map<std::wstring, Character>::iterator iter = m_Characters.begin(); iter != m_Characters.end(); iter++)
    {
        (*iter).second.Update(delta_time);
    }
}

void GameState::AddCharacter(Character character)
{
    m_Characters.emplace(std::make_pair(character.GetID(), character));
}

void GameState::SetPlayer(sf::String ID)
{
    // TODO: Transition to using only IDs
    m_Player = &m_Characters.at(ID.toWideString());
    debug("Set player to %p", &m_Player);
}

void GameState::MovePlayer(Direction dir)
{
    if (m_Player != NULL) {
        m_Player->Move(dir);
    }
}

sf::Vector2f GameState::GetPlayerPixelPosition()
{
    return m_Player->GetPixelPosition();
}

void GameState::LoadMap(sf::String path)
{
    //TODO: Figure out what to do in re: player not being on this map, etc
    //TODO: Make this execute a script pre-loading
    if (m_CurrentMap != NULL) {
        debug("Deleting (from memory, don't worry) previously loaded map at %p", m_CurrentMap);
        delete m_CurrentMap;
    }
    m_CurrentMapPath = path;
    m_CurrentMap = new tmx::TileMap(path.toAnsiString());
    m_CurrentMap->ShowObjects();
    
    debug("Loaded map from %s", path.toAnsiString().c_str());
}

void GameState::ShowLowerLayers() const
{
    // Show all the lower layers that exist
    // TODO: When/if STP gets fixed, allow as many as 10 under and over layers, and as few as 0, by using if (check_layer_somehow) { show/hide }
    const std::string lower_layers[] = {"under0", "under1", "under2", "under3", "under4", "under5", "under6", "under7", "under8", "under9"};
    for (int i = 0; i < 10; i++){
        // Check if the layer exists; if the pointer to the ref is null, it isn't real.
        if (&m_CurrentMap->GetLayer(lower_layers[i]) != NULL) {
            m_CurrentMap->GetLayer(lower_layers[i]).visible = true;
        }
    }
    
    // Hide all the upper layers
    const std::string upper_layers[] = {"over0", "over1", "over2", "over3", "over4", "over5", "over6", "over7", "over8", "over9"};
    for (int i = 0; i < 10; i++){
        // Check if the layer exists; if the pointer to the ref is null, it isn't real.
        if (&m_CurrentMap->GetLayer(upper_layers[i]) != NULL) {
            m_CurrentMap->GetLayer(upper_layers[i]).visible = false;
        }
    }
}

void GameState::ShowUpperLayers() const
{
    // Hide all the lower layers that exist
    // TODO: When/if STP gets fixed, allow as many as 10 under and over layers, and as few as 0, by using if (check_layer_somehow) { show/hide }
    const std::string lower_layers[] = {"under0", "under1", "under2", "under3", "under4", "under5", "under6", "under7", "under8", "under9"};
    for (int i = 0; i < 10; i++){
        // Check if the layer exists; if the pointer to the ref is null, it isn't real.
        if (&m_CurrentMap->GetLayer(lower_layers[i]) != NULL) {
            m_CurrentMap->GetLayer(lower_layers[i]).visible = false;
        }
    }
    
    // Show all the upper layers
    const std::string upper_layers[] = {"over0", "over1", "over2", "over3", "over4", "over5", "over6", "over7", "over8", "over9"};
    for (int i = 0; i < 10; i++){
        // Check if the layer exists; if the pointer to the ref is null, it isn't real.
        if (&m_CurrentMap->GetLayer(upper_layers[i]) != NULL) {
            m_CurrentMap->GetLayer(upper_layers[i]).visible = true;
        }
    }
}

void GameState::draw(sf::RenderTarget& target, sf::RenderStates states) const
{
    // Draw the ground - the map's lower layers, in other words.
    ShowLowerLayers();
    target.draw(*m_CurrentMap);
    
    // Draw objects, characters, mobs, et cetera
    // Iterate through Characters, updating each
    for (std::unordered_map<std::wstring, Character>::const_iterator iter = m_Characters.begin(); iter != m_Characters.end(); iter++)
    {
        target.draw((*iter).second);
    }
    
    // Draw the upper map layers
    ShowUpperLayers();
    target.draw(*m_CurrentMap);
    
    // UI will be drawn after this, in another object, so we don't have to worry about it here.
    
}
