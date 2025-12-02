using Microsoft.Xna.Framework.Graphics;
using Microsoft.Xna.Framework;

namespace FactoryGame.Core;

/// <summary>
/// Provides helper methods for creating textures.
/// </summary>
public static class Texture2DHelper
{
    private static Texture2D? whiteTexture;

    /// <summary>
    /// Gets a 1x1 white texture for drawing rectangles.
    /// </summary>
    public static Texture2D GetWhiteTexture(GraphicsDevice graphicsDevice)
    {
        if (whiteTexture == null)
        {
            whiteTexture = new Texture2D(graphicsDevice, 1, 1);
            whiteTexture.SetData(new[] { Color.White });
        }
        return whiteTexture;
    }
}
