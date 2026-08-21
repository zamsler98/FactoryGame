public class Chunk {
    public Position Position { get; private set; }
    private List<Entity> _entities = new List<Entity>();


    public void AddEntity(Entity entity) {
        _entities.Add(entity)
    }

    public void Render() {

        foreach (var entity in _entities) {
            //Get entity renderer from global collection
            //Render entities
        }
    }
}


//Interface for rendering an entity. Not exactly sure what 
public interface IEntityRenderer<TEntity> where TEntity : Entity {
    pubilc void Render(TEntity entity);
}

//Global collection of entity render objects so that each entity type will use the same entity
//renderer object. This allows us to have 10,000 entities all use the same texture object
public RendererCollection {
    public IEntityRenderer<TEntity> GetRenderer<TEntity>() where TEntity : Entity {
        //returns the entity renderer object for the entity type        
    }
}


pub trai
