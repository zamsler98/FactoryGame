using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Input;

namespace FactoryGame.Core;

/// <summary>
/// Centralizes input handling for keyboard and mouse input.
/// </summary>
public class InputManager
{
    private KeyboardState previousKeyboardState;
    private MouseState previousMouseState;

    /// <summary>
    /// Updates the input manager state. Should be called once per frame.
    /// </summary>
    public void Update()
    {
        previousKeyboardState = Keyboard.GetState();
        previousMouseState = Mouse.GetState();
    }

    /// <summary>
    /// Checks if a key was just pressed this frame.
    /// </summary>
    /// <param name="key">The key to check.</param>
    /// <returns>True if the key was just pressed, false otherwise.</returns>
    public bool IsKeyPressed(Keys key)
    {
        var currentKeyboardState = Keyboard.GetState();
        return currentKeyboardState.IsKeyDown(key) && previousKeyboardState.IsKeyUp(key);
    }

    /// <summary>
    /// Checks if a key is currently held down.
    /// </summary>
    /// <param name="key">The key to check.</param>
    /// <returns>True if the key is held down, false otherwise.</returns>
    public bool IsKeyDown(Keys key)
    {
        return Keyboard.GetState().IsKeyDown(key);
    }

    /// <summary>
    /// Checks if the left mouse button was just clicked this frame.
    /// </summary>
    /// <returns>True if left mouse button was just clicked, false otherwise.</returns>
    public bool IsLeftMouseClicked()
    {
        var currentMouseState = Mouse.GetState();
        return currentMouseState.LeftButton == ButtonState.Pressed && 
               previousMouseState.LeftButton == ButtonState.Released;
    }

    /// <summary>
    /// Checks if the right mouse button was just clicked this frame.
    /// </summary>
    /// <returns>True if right mouse button was just clicked, false otherwise.</returns>
    public bool IsRightMouseClicked()
    {
        var currentMouseState = Mouse.GetState();
        return currentMouseState.RightButton == ButtonState.Pressed && 
               previousMouseState.RightButton == ButtonState.Released;
    }

    /// <summary>
    /// Gets the current mouse position.
    /// </summary>
    /// <returns>The current mouse position as a Vector2.</returns>
    public Vector2 GetMousePosition()
    {
        var mouseState = Mouse.GetState();
        return new Vector2(mouseState.X, mouseState.Y);
    }

    /// <summary>
    /// Gets the current mouse state.
    /// </summary>
    /// <returns>The current mouse state.</returns>
    public MouseState GetMouseState()
    {
        return Mouse.GetState();
    }

    /// <summary>
    /// Gets the current keyboard state.
    /// </summary>
    /// <returns>The current keyboard state.</returns>
    public KeyboardState GetKeyboardState()
    {
        return Keyboard.GetState();
    }

    /// <summary>
    /// Checks if the escape key was pressed (commonly used for exiting).
    /// </summary>
    /// <returns>True if escape was pressed, false otherwise.</returns>
    public bool IsExitPressed()
    {
        return IsKeyPressed(Keys.Escape);
    }

    /// <summary>
    /// Gets movement input from arrow keys or WASD.
    /// </summary>
    /// <returns>A normalized Vector2 representing movement direction.</returns>
    public Vector2 GetMovementInput()
    {
        var movement = Vector2.Zero;

        if (IsKeyDown(Keys.Left) || IsKeyDown(Keys.A))
        {
            movement.X -= 1;
        }
        if (IsKeyDown(Keys.Right) || IsKeyDown(Keys.D))
        {
            movement.X += 1;
        }
        if (IsKeyDown(Keys.Up) || IsKeyDown(Keys.W))
        {
            movement.Y -= 1;
        }
        if (IsKeyDown(Keys.Down) || IsKeyDown(Keys.S))
        {
            movement.Y += 1;
        }

        if (movement != Vector2.Zero)
        {
            movement.Normalize();
        }

        return movement;
    }
}