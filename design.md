=============
Tetris Game
=============

Test to help me learn bevy / rust. Make a tetris clone.


Plan
----
☑☐
☑1. Make a small rectangle appear on the screen
☑2. Make a rectangle fall from the top of the screen to the bottom
☐3. Allow user to move it while it is in mid-air
☑4. Make blocks drop repeatedly
☑5. Make blocks collide with each other
☐6. Create new block shapes
☐7. Allow user to rotate blocks in mid-air
☐8. Make game end when blocks hit top
☐9. Create a scoring system
☐10. Save scores

Make a small rectangle appear on the screen
-------------------------------------------
Use a mesh / Mesh2D?

Make a rectangle fall from the top of the screen to the bottom
--------------------------------------------------------------
move_blocks_down system drops block and checks for collision / reaching the
bottom. Probably need to move collision detection to new function for more
complex shapes and then use something like rapier for doing detection and maybe
dropping? Rapier is a physics engine with a bevy plugin.

Allow user to move it while it is in mid-air
--------------------------------------------
How to do user input?
Need to mark one block as the "current" block? There should only be one block
    falling at a time and that is the current block.
Make system that checks for user input and modifies transform of current block
    to move it left or right.

Make blocks drop repeatedly
---------------------------
spawn_blocks_periodically system uses BlockSpawnTimer to know when to spawn a
new block

Make blocks collide with each other
------------------------------------
Should switch to rapier for using more complex shapes

Create new block shapes
-----------------------

Allow user to rotate blocks in mid-air
--------------------------------------

Make game end when blocks hit top
----------------------------------
Probably do this in the move_blocks_down system or spawn_blocks_periodically
Just check if the current block stops above a threshold

Create a scoring system
------------------------
Score == number of blocks placed?

 Save scores
-------------
When the game ends, prompt for initials and save score in leaderboard
