using System;
using FactoryGame.Core.Localization;
using System.Collections.Generic;
using System.Globalization;
using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Graphics;
using Microsoft.Xna.Framework.Input;
using static System.Net.Mime.MediaTypeNames;

namespace FactoryGame.Core
{
    /// <summary>
    /// The main class for the game, responsible for managing game components, settings, 
    /// and platform-specific configurations.
    /// </summary>
    public class FactoryGameGame : Game
    {
        // Resources for drawing.
        private GraphicsDeviceManager graphicsDeviceManager;

        private SpriteBatch spriteBatch;
        private Player player;
        private SpriteFont debugFont;

        // Building placement system
        private Grid grid;
        private BuildingManager buildingManager;
        private BuildingType selectedBuildingType = BuildingType.Factory;
        private MouseState previousMouseState;

        // Grid overlay system
        private GridOverlay gridOverlay;

        /// <summary>
        /// Indicates if the game is running on a mobile platform.
        /// </summary>
        public readonly static bool IsMobile = OperatingSystem.IsAndroid() || OperatingSystem.IsIOS();

        /// <summary>
        /// Indicates if the game is running on a desktop platform.
        /// </summary>
        public readonly static bool IsDesktop = OperatingSystem.IsMacOS() || OperatingSystem.IsLinux() || OperatingSystem.IsWindows();


        /// <summary>
        /// Initializes a new instance of the game. Configures platform-specific settings, 
        /// initializes services like settings and leaderboard managers, and sets up the 
        /// screen manager for screen transitions.
        /// </summary>
        public FactoryGameGame()
        {
            graphicsDeviceManager = new GraphicsDeviceManager(this);
            graphicsDeviceManager.PreferredBackBufferWidth = 800;
            graphicsDeviceManager.PreferredBackBufferHeight = 600;


            // Share GraphicsDeviceManager as a service.
            Services.AddService(typeof(GraphicsDeviceManager), graphicsDeviceManager);

            Content.RootDirectory = "Content";

            // Configure screen orientations.
            graphicsDeviceManager.SupportedOrientations = DisplayOrientation.LandscapeLeft | DisplayOrientation.LandscapeRight;


        }


        /// <summary>
        /// Initializes the game, including setting up localization and adding the 
        /// initial screens to the ScreenManager.
        /// </summary>
        protected override void Initialize()
        {
            base.Initialize();

            // Load supported languages and set the default language.
            List<CultureInfo> cultures = LocalizationManager.GetSupportedCultures();
            var languages = new List<CultureInfo>();
            for (int i = 0; i < cultures.Count; i++)
            {
                languages.Add(cultures[i]);
            }

            // TODO You should load this from a settings file or similar,
            // based on what the user or operating system selected.
            var selectedLanguage = LocalizationManager.DEFAULT_CULTURE_CODE;
            LocalizationManager.SetCulture(selectedLanguage);


        }


        /// <summary>
        /// Loads game content, such as textures and particle systems.
        /// </summary>
        protected override void LoadContent()
        {
            base.LoadContent();
            spriteBatch = new SpriteBatch(GraphicsDevice);
            // Always initialize player here to ensure texture is created with valid GraphicsDevice
            player = new Player(new Vector2(0, 0));
            player.LoadContent(GraphicsDevice);
            debugFont = Content.Load<SpriteFont>("Fonts/DebugFont");

            // Initialize grid and building manager
            grid = new Grid();
            buildingManager = new BuildingManager(grid);

            // Initialize grid overlay
            gridOverlay = new GridOverlay(GraphicsDevice);
            gridOverlay.LoadContent();
        }



        /// <summary>
        /// Updates the game's logic, called once per frame.
        /// </summary>
        /// <param name="gameTime">
        /// Provides a snapshot of timing values used for game updates.
        /// </param>
        protected override void Update(GameTime gameTime)
        {
            // Exit the game if the Back button (GamePad) or Escape key (Keyboard) is pressed.
            if (GamePad.GetState(PlayerIndex.One).Buttons.Back == ButtonState.Pressed
                || Keyboard.GetState().IsKeyDown(Keys.Escape))
                Exit();

            // Building type selection
            var keyboardState = Keyboard.GetState();
            if (keyboardState.IsKeyDown(Keys.F))
            {
                selectedBuildingType = BuildingType.Factory;
            }
            else if (keyboardState.IsKeyDown(Keys.C))
            {
                selectedBuildingType = BuildingType.Conveyor;
            }

            // Grid overlay toggle
            if (keyboardState.IsKeyDown(Keys.G))
            {
                gridOverlay.ToggleVisibility();
            }

            // Mouse input for placement
            var mouseState = Mouse.GetState();
            if (mouseState.LeftButton == ButtonState.Pressed && previousMouseState.LeftButton == ButtonState.Released)
            {
                int gridX = mouseState.X / Grid.CellSize;
                int gridY = mouseState.Y / Grid.CellSize;
                buildingManager.TryPlaceBuilding(selectedBuildingType, gridX, gridY);
            }
            // Mouse input for deletion
            if (mouseState.RightButton == ButtonState.Pressed && previousMouseState.RightButton == ButtonState.Released)
            {
                int gridX = mouseState.X / Grid.CellSize;
                int gridY = mouseState.Y / Grid.CellSize;
                if (grid.GetBuilding(gridX, gridY) != null)
                {
                    buildingManager.RemoveBuilding(gridX, gridY);
                }
            }

            previousMouseState = mouseState;

            // TODO: Add your update logic here
            player?.Update(gameTime);

            base.Update(gameTime);
        }



        /// <summary>
        /// Draws the game's graphics, called once per frame.
        /// </summary>
        /// <param name="gameTime">
        /// Provides a snapshot of timing values used for rendering.
        /// </param>
        protected override void Draw(GameTime gameTime)
        {
            // Clears the screen with the MonoGame orange color before drawing.
            GraphicsDevice.Clear(Color.Black);

            spriteBatch.Begin();

            // Draw grid
            for (int x = 0; x < Grid.Width; x++)
            {
                for (int y = 0; y < Grid.Height; y++)
                {
                    var rect = new Rectangle(x * Grid.CellSize, y * Grid.CellSize, Grid.CellSize, Grid.CellSize);
                    spriteBatch.Draw(Texture2DHelper.GetWhiteTexture(GraphicsDevice), rect, Color.DarkGray * 0.2f);
                }
            }

            // Draw grid overlay with hover highlighting
            var mouseState = Mouse.GetState();
            var mousePosition = new Vector2(mouseState.X, mouseState.Y);
            gridOverlay.Draw(spriteBatch, grid, mousePosition);

            // Draw buildings
            for (int x = 0; x < Grid.Width; x++)
            {
                for (int y = 0; y < Grid.Height; y++)
                {
                    var building = grid.GetBuilding(x, y);
                    if (building != null)
                    {
                        var rect = new Rectangle(x * Grid.CellSize, y * Grid.CellSize, Grid.CellSize, Grid.CellSize);
                        Color color = building.Type == BuildingType.Factory ? Color.Blue : Color.Green;
                        spriteBatch.Draw(Texture2DHelper.GetWhiteTexture(GraphicsDevice), rect, color * 0.7f);
                    }
                }
            }

            // Draw selected building type info
            spriteBatch.DrawString(debugFont, $"Selected: {selectedBuildingType}", new Vector2(10, 10), Color.White);

            player?.Draw(spriteBatch);
            spriteBatch.End();

            base.Draw(gameTime);
        }




    }
}