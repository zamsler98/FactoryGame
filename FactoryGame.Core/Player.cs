using System;
using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Graphics;
using Microsoft.Xna.Framework.Input;


namespace FactoryGame.Core;

/// <summary>
/// Represents the player character and handles movement and rendering.
/// </summary>
public class Player : IDisposable
{
    private Texture2D? texture;
    private Vector2 position;
    private readonly float speed = GameConstants.PlayerSpeed;
    private const int TextureSize = GameConstants.PlayerTextureSize;
    private bool disposed = false;

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
        texture = new Texture2D(graphicsDevice, TextureSize, TextureSize);
        Color[] data = new Color[TextureSize * TextureSize];
        for (int i = 0; i < data.Length; i++)
        {
            data[i] = Color.White;
        }
        texture.SetData(data);
    }



    /// <summary>
    /// Updates the player's position based on keyboard input.
    /// </summary>
    public void Update(GameTime gameTime, InputManager inputManager)
    {
        var movement = inputManager.GetMovementInput();

        if (movement != Vector2.Zero)
        {
            var newPosition = position + movement * speed * (float)gameTime.ElapsedGameTime.TotalSeconds;
            
            // Boundary checking - keep player within game window
            newPosition.X = Math.Max(0, Math.Min(newPosition.X, GameConstants.WindowWidth - TextureSize));
            newPosition.Y = Math.Max(0, Math.Min(newPosition.Y, GameConstants.WindowHeight - TextureSize));
            
            position = newPosition;
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

    /// <summary>
    /// Releases all resources used by the Player.
    /// </summary>
    public void Dispose()
    {
        Dispose(true);
        GC.SuppressFinalize(this);
    }

    /// <summary>
    /// Releases the unmanaged resources used by the Player and optionally releases the managed resources.
    /// </summary>
    /// <param name="disposing">true to release both managed and unmanaged resources; false to release only unmanaged resources.</param>
    protected virtual void Dispose(bool disposing)
    {
        if (!disposed)
        {
            if (disposing)
            {
                texture?.Dispose();
                texture = null;
            }
            disposed = true;
        }
    }

    /// <summary>
    /// Destructor for Player.
    /// </summary>
    ~Player()
    {
        Dispose(false);
    }
}