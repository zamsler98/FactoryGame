namespace FactoryGame.Core;

/// <summary>
/// Represents a building placed on the grid.
/// </summary>
public class Building
{
    public BuildingType Type { get; }
    public int GridX { get; }
    public int GridY { get; }

    public Building(BuildingType type, int gridX, int gridY)
    {
        Type = type;
        GridX = gridX;
        GridY = gridY;
    }
}
