namespace FactoryGame.Core;

/// <summary>
/// Represents the game world grid for building placement.
/// </summary>
public class Grid
{
    public const int CellSize = 32;
    public const int Width = 20;
    public const int Height = 15;

    private readonly Building?[,] cells = new Building[Width, Height];

    /// <summary>
    /// Checks if a cell is occupied by a building.
    /// </summary>
    public bool IsCellOccupied(int x, int y)
    {
        if (x < 0 || x >= Width || y < 0 || y >= Height)
        {
            return false;
        }
        return cells[x, y] != null;
    }


    /// <summary>
    /// Places a building at the specified grid cell if unoccupied.
    /// </summary>
    public bool PlaceBuilding(Building building, int x, int y)
    {
        if (x < 0 || x >= Width || y < 0 || y >= Height)
        {
            return false;
        }
        if (cells[x, y] != null)
        {
            return false;
        }
        cells[x, y] = building;
        return true;
    }

    /// <summary>
    /// Gets the building at the specified cell, or null if empty.
    /// </summary>
    public Building? GetBuilding(int x, int y)
    {
        return cells[x, y];
    }
}
