using System;
using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Graphics;
using Microsoft.Xna.Framework.Input;


namespace FactoryGame.Core;

/// <summary>
/// Represents the player character and handles movement and rendering.
/// </summary>
public class Player
{
    private Texture2D texture;
    private Vector2 position;
    private readonly float speed = 200f; // pixels per second

    /// <summary>
    /// Initializes a new instance of the Player class.
    /// </summary>
    /// <param name="startPosition">Initial position of the player.</param>
    public Player(Vector2 startPosition)
    {
        position = startPosition;
    }

    /// <summary>
    /// Loads the player texture.
    /// </summary>
    public void LoadContent(GraphicsDevice graphicsDevice)
    {
        texture = new Texture2D(graphicsDevice, 64, 64);
        Color[] data = new Color[64 * 64];
        for (int i = 0; i < data.Length; i++)
        {
            data[i] = Color.White;
        }
        texture.SetData(data);
    }



    /// <summary>
    /// Updates the player's position based on keyboard input.
    /// </summary>
    public void Update(GameTime gameTime)
    {
        var keyboardState = Keyboard.GetState();
        var movement = Vector2.Zero;

        if (keyboardState.IsKeyDown(Keys.Left) || keyboardState.IsKeyDown(Keys.A))
        {
            movement.X -= 1;
        }
        if (keyboardState.IsKeyDown(Keys.Right) || keyboardState.IsKeyDown(Keys.D))
        {
            movement.X += 1;
        }
        if (keyboardState.IsKeyDown(Keys.Up) || keyboardState.IsKeyDown(Keys.W))
        {
            movement.Y -= 1;
        }
        if (keyboardState.IsKeyDown(Keys.Down) || keyboardState.IsKeyDown(Keys.S))
        {
            movement.Y += 1;
        }

        if (movement != Vector2.Zero)
        {
            movement.Normalize();
            position += movement * speed * (float)gameTime.ElapsedGameTime.TotalSeconds;
        }
    }

    /// <summary>
    /// Draws the player character.
    /// </summary>
    public void Draw(SpriteBatch spriteBatch)
    {
        if (texture != null)
        {
            spriteBatch.Draw(texture, position, Color.Red);
        }
    }
}
