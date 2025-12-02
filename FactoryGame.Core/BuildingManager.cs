namespace FactoryGame.Core;

/// <summary>
/// Manages building placement and collision checks.
/// </summary>
public class BuildingManager
{
    private readonly Grid grid;

    public BuildingManager(Grid grid)
    {
        this.grid = grid;
    }

    /// <summary>
    /// Attempts to place a building of the specified type at the given grid cell.
    /// </summary>
    public bool TryPlaceBuilding(BuildingType type, int x, int y)
    {
        if (grid.IsCellOccupied(x, y))
        {
            return false;
        }
        var building = new Building(type, x, y);
        return grid.PlaceBuilding(building, x, y);
    }

    /// <summary>
    /// Removes the building at the specified grid cell, if any.
    /// </summary>
    public bool RemoveBuilding(int x, int y)
    {
        return grid.RemoveBuilding(x, y);
    }
}
