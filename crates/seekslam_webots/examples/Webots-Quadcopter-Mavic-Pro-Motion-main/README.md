# Webots-Quadcopter-Mavic-Pro-Planar-Motion

This repository uses Webots to simulate the DJI-Mavic Pro Quadcopter. 

Note : The PID controller and the PID parameters used are referenced from : https://github.com/PrasadNR/Webots-Quadcopter-Python-SITL

a) This controller simulates the motion of the Quadcopter for a set of present and target coordinates in 3D space.

b) For present and target coordinates lying on the ground plane, the quadcopter moves in a semi-circular trajecotry in the altitude space, about a centre lying on the ground plane.

c) The circular path is obtained using the parametric form of a circle, centered at the average of the present and target, (x,z) coordinates.

d) The time taken by the quadcopter to move in the semi circular path can be controlled by the variable 'n', which represents the number of time steps.

e) The quadcopter moves along the arc of the semicircle in small steps defined by the angle subtended by the radius at the present coordinates, with the radius at the target coordinates; sweeping a full semicircle from 180 deg to 0 deg.

f) The path is plotted using matplotlib library.
