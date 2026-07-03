# Refactor plan: move startup and shared constants into the startup module

## Goal
Move the shared gameplay constants and the startup/setup logic out of the main entrypoint and into the startup module so the project structure is clearer and the startup flow is easier to maintain.

## Target outcome
- The constants currently defined in [src/not_bevy/spawn_sprites.rs](src/not_bevy/spawn_sprites.rs) become shared definitions in [src/not_bevy/constants_and_startup.rs](src/not_bevy/constants_and_startup.rs).
- The startup logic currently implemented in [src/main.rs](src/main.rs) is moved into the startup module as a reusable function.
- [src/main.rs](src/main.rs) becomes a thinner entrypoint that wires the app together and calls the startup module.

## Proposed steps

1. Create the shared module surface
   - Add the necessary exports in [src/not_bevy/mod.rs](src/not_bevy/mod.rs) so the startup module is available to the rest of the app.
   - Make sure [src/not_bevy/constants_and_startup.rs](src/not_bevy/constants_and_startup.rs) exposes the constants and startup function that [src/main.rs](src/main.rs) will import.

2. Move shared constants
   - Copy the canvas, player, spawn-distance, and level-size constants from [src/not_bevy/spawn_sprites.rs](src/not_bevy/spawn_sprites.rs) into [src/not_bevy/constants_and_startup.rs](src/not_bevy/constants_and_startup.rs).
   - Keep the names the same initially to reduce churn and make the refactor easier to verify.
   - Update imports so [src/not_bevy/spawn_sprites.rs](src/not_bevy/spawn_sprites.rs) uses the constants from the startup module instead of defining them locally.

3. Move startup logic
   - Move the current `startup` function from [src/main.rs](src/main.rs) into [src/not_bevy/constants_and_startup.rs](src/not_bevy/constants_and_startup.rs).
   - Keep the behavior identical at first; this is a structural refactor rather than a gameplay change.
   - If needed, rename it to something like `build_world` or `setup_scene` for clarity, but preserve the public call site in [src/main.rs](src/main.rs).

4. Rebuild the import path
   - In [src/main.rs](src/main.rs), replace the local startup implementation with an import from the startup module.
   - Remove duplicated imports that are now owned by the startup module.
   - Ensure the app still calls the startup function from the `Startup` schedule.

5. Keep the module responsibilities clean
   - Let [src/not_bevy/constants_and_startup.rs](src/not_bevy/constants_and_startup.rs) own:
     - world constants
     - initial scene setup
     - animation configuration used during startup
     - player spawn setup and camera initialization
   - Keep [src/not_bevy/spawn_sprites.rs](src/not_bevy/spawn_sprites.rs) focused on runtime object spawning and furniture behavior.

6. Verify after the move
   - Run the project and confirm the scene still loads and the player spawns correctly.
   - Check for compile errors caused by moved imports, missing visibility, or renamed symbols.
   - Confirm that the constants are still used by the relevant systems after the move.

## Suggested implementation order
1. Move constants into the startup module.
2. Move the startup function into the startup module.
3. Update imports and call sites.
4. Run the game and fix any fallout.

## Acceptance criteria
- The app still builds and runs after the refactor.
- The constants are defined in one place.
- The startup code lives in the startup module instead of the main entrypoint.
- The game behavior is unchanged.
