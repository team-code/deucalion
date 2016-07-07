#pragma once

#include <SFML/System.hpp>
#include <SFML/Graphics.hpp>

/*
 * The UI State holds a set of objects like 
 *      UserInterfaceWindow, a rectangle drawn in the selected UI style
 *      UserInterfaceText,  a bit of text drawn in the selected UI style
 *      etc
 *  and draws them on command

class UserInterfaceState : sf::Drawable
{
public:
    UserInterfaceState();
    ~UserInterfaceState();

};

