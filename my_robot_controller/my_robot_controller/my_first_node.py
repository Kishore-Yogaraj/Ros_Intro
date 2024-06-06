#!/usr/bin/env python3
import rclpy
from rclpy.node import Node

#Node class for programmatical purpose
class MyNode(Node):

    def __init__(self):
        super().__init__("first_node") #Node name
        self.counter_ = 0
        self.create_timer(1.0, self.timer_callback)

    def timer_callback(self):
        self.get_logger().info("Hello" + str(self.counter_))
        self.counter_ += 1

def main(args=None):
    rclpy.init(args=args) #Initialize ros2 communications
    node = MyNode() #Node create in main which is inherited by the node class from rclpy
    rclpy.spin(node) #Spin functionality will have the node continuously run until it is killed
    rclpy.shutdown()  #Shutdown ros2 communications

if __name__ == '__main__':
    main()

    