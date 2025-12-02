using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Graphics;
using System;

namespace FactoryGame.Core;

/// <summary>
/// Handles rendering of the grid overlay with hover highlighting.
/// </summary>
public class GridOverlay
{
    private readonly GraphicsDevice graphicsDevice;
    private bool isVisible = true;
    private readonly Color gridColor = Color.White * 0.3f;
    private readonly Color highlightColor = Color.Yellow * 0.5f;
    private Texture2D whiteTexture;

    /// <summary>
    /// Gets or sets whether the grid overlay is visible.
    /// </summary>
    public bool IsVisible
    {
        get => isVisible;
        set => isVisible = value;
    }

    /// <summary>
    /// Initializes a new instance of the GridOverlay class.
    /// </summary>
    /// <param name="graphicsDevice">The graphics device for rendering.</param>
    public GridOverlay(GraphicsDevice graphicsDevice)
    {
        this.graphicsDevice = graphicsDevice ?? throw new ArgumentNullException(nameof(graphicsDevice));
    }

    /// <summary>
    /// Loads content required for the grid overlay.
    /// </summary>
    public void LoadContent()
    {
        whiteTexture = Texture2DHelper.GetWhiteTexture(graphicsDevice);
    }

    /// <summary>
    /// Toggles the visibility of the grid overlay.
    /// </summary>
    public void ToggleVisibility()
    {
        isVisible = !isVisible;
    }

    /// <summary>
    /// Draws the grid overlay with hover highlighting.
    /// </summary>
    /// <param name="spriteBatch">The sprite batch to use for drawing.</param>
    /// <param name="grid">The game grid.</param>
    /// <param name="mousePosition">The current mouse position.</param>
    public void Draw(SpriteBatch spriteBatch, Grid grid, Vector2 mousePosition)
    {
        if (!isVisible || spriteBatch == null || grid == null)
        {
            return;
        }

        DrawGridLines(spriteBatch, grid);
        DrawHoverHighlight(spriteBatch, grid, mousePosition);
    }

    /// <summary>
    /// Draws the grid lines.
    /// </summary>
    private void DrawGridLines(SpriteBatch spriteBatch, Grid grid)
    {
        int cellSize = Grid.CellSize;
        int gridWidth = Grid.Width * cellSize;
        int gridHeight = Grid.Height * cellSize;

        // Draw vertical lines
        for (int x = 0; x <= Grid.Width; x++)
        {
            int lineX = x * cellSize;
            var lineRect = new Rectangle(lineX, 0, 1, gridHeight);
            spriteBatch.Draw(whiteTexture, lineRect, gridColor);
        }

        // Draw horizontal lines
        for (int y = 0; y <= Grid.Height; y++)
        {
            int lineY = y * cellSize;
            var lineRect = new Rectangle(0, lineY, gridWidth, 1);
            spriteBatch.Draw(whiteTexture, lineRect, gridColor);
        }
    }

    /// <summary>
    /// Draws the hover highlight for the cell under the mouse.
    /// </summary>
    private void DrawHoverHighlight(SpriteBatch spriteBatch, Grid grid, Vector2 mousePosition)
    {
        // Convert mouse position to grid coordinates
        int gridX = (int)(mousePosition.X / Grid.CellSize);
        int gridY = (int)(mousePosition.Y / Grid.CellSize);

        // Check if the mouse is within grid bounds
        if (gridX >= 0 && gridX < Grid.Width && gridY >= 0 && gridY < Grid.Height)
        {
            var highlightRect = new Rectangle(
                gridX * Grid.CellSize,
                gridY * Grid.CellSize,
                Grid.CellSize,
                Grid.CellSize
            );
            spriteBatch.Draw(whiteTexture, highlightRect, highlightColor);
        }
    }
}