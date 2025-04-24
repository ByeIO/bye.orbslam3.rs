from controller import Supervisor
import sys

class SupervisorLight:
    def __init__(self):
        # Simulation Parameters
        self.time_step = 32 # (ms)
        self.time_light = 60 # Change this to 60 for 60 seconds
        self.flag_light = 1 # Used to identify the current position of the light node
        
        # Initiate Supervisor Module
        self.supervisor = Supervisor()
        
        # Get the robot node from your world environment
        self.robot_node = self.supervisor.getFromDef("Controller")
        if self.robot_node is None:
            sys.stderr.write("No DEF Controller node found in the current world file\n")
            sys.exit(1)
        
        # Get the rotation and translation fields from your robot node
        self.trans_field = self.robot_node.getField("translation")
        self.rot_field = self.robot_node.getField("rotation")
        
        # Get the light node from your world environment
        self.light_node = self.supervisor.getFromDef("Light")
        if self.light_node is None:
            sys.stderr.write("No DEF Light node found in the current world file\n")
            sys.exit(1)
        
        # Get the location and direction fields from the light node          
        self.location_field = self.light_node.getField("location")
        self.direction_field = self.light_node.getField("direction")
        
        # Define different light positions for toggling
        self.light_positions = [
            [1.0, 2.0, 0.5],   # Position 1
            [2.0, 2.0, 0.5],   # Position 2
            [3.0, 2.0, 0.5],   # Position 3
            [4.0, 2.0, 0.5]    # Position 4
        ]

    def run_seconds(self, seconds):
        # Calculate the number of iterations based on the time_step of the simulator 
        stop = int((seconds * 1000) / self.time_step)
        iterations = 0  # Reset the counter

        while self.supervisor.step(self.time_step) != -1:
            if iterations == stop:  # Every 60 seconds
                iterations = 0  # Reset the counter

                # Reset robot's position and rotation
                INITIAL_TRANS = [0.35, 0.20, 0]
                self.trans_field.setSFVec3f(INITIAL_TRANS)
                INITIAL_ROT = [0, 1, 0, -0.0]
                self.rot_field.setSFRotation(INITIAL_ROT)
                self.robot_node.resetPhysics()

                # Update light's position
                new_position = self.light_positions[self.flag_light % len(self.light_positions)]
                self.location_field.setSFVec3f(new_position)
                print(f"Light moved to: {new_position}")

                # Increment flag to change light's position next time
                self.flag_light += 1

            # Increment the counter
            iterations += 1

    def run_demo(self):
        # Reset robot's position and rotation
        INITIAL_TRANS = [0.35, 0.20, 0]
        self.trans_field.setSFVec3f(INITIAL_TRANS)
        INITIAL_ROT = [0, 1, 0, -0.0]
        self.rot_field.setSFRotation(INITIAL_ROT)
        self.robot_node.resetPhysics()
        
        # Run the loop to update the light position every 60 seconds
        self.run_seconds(self.time_light)
    
if __name__ == "__main__":
    # Create Supervisor Controller
    model = SupervisorLight()
    # Run Supervisor Controller
    model.run_demo()
