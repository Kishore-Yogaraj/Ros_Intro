from launch import LaunchDescription
from launch_ros.actions import Node

#Defines how the nodes should be launched and when and where they should be launched

# Name of topic to be subscribed or published to
TOPIC = "chatter"

def generate_launch_description():
    talker = Node(
        package = "talker_listener", #package name that node is in 
        executable = "talkerNode", #executable in the setup.py file
        name = "talker_node", #name of node given for when you want to launch it

        # How to initialize a parameter for each node
        parameters=[{
            "topic": TOPIC #key:value
        }]
    )

    listener = Node(
        package = "talker_listener",
        executable="listenerNode",
        name="listener_node",
        parameters=[{
            "topic": TOPIC
        }]
    )

    return LaunchDescription([
        talker,
        listener
    ])
