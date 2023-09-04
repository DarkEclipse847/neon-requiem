# neon-requiem
Oddysey for love and purpouse
## Isometric Quest using Bevy 0.11

This is a simple visual novel(quest) about finding purpouse in life. This is a story about a musician who is fighting with depression and existencial crysis.

### Tasks:
- [ ] Need to implement 3d(fake2d) walls
    - [x] draw a texture for 3d mesh
    - [ ] create collision system
    - [ ] draw some sprites so walls will be more realistic 
- [ ] Implement dynamic lightning
    - [ ] add lightsourses
    - [ ] add glowing characters(so they can be visible in the dark)
    - [ ] add glowing blocks
- [ ] Find a way to load assets from separate files(maybe bevy_asset_loader?) cause it's hard to keep code clean with random integers which define indexes of sprite/tile
    - [ ] check out bevy_asset_loader
- [x] ~~Create a camera movement~~
    - [x] ~~move the camera up~~
    - [x] ~~Facing camera is working differently than before, fix it~~
- [x] ~~Fix movement animation~~
    - ~~animation behaving weirdly after adding face_camera fn so it needs to be fixed~~
- [x] ~~Tiles are larger than expected. Seems like it fills 2 by 2 sqare with texture. Need to fix this~~
### Thoughts:
- Overlapping floor textures actually looks really cool, i need to run tests on other devices later, maybe i'll add thisartifacts as a feature

- Use bevy-vfx-bug plugin to complete trippy scene in the future
    - chromatic abberation could come in handy for replecating visual glitches
    