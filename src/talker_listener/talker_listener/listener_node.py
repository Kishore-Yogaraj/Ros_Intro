import rclpy
from rclpy.node import Node

from std_msgs.msg import String #Subscribes to topic that uses message type string so has to be specified


class ListenerNode(Node):
    def __init__(self):
        super().__init__("listener_node")
        self.subscription = self.create_subscription(String, 'topic', self.listener_callback, 10)

    def listener_callback(self,msg):
        self.get_logger().info(f"Receiver {msg.data}")


def main(args=None):
    rclpy.init(args=args)

    #create node
    listenerNode = ListenerNode()

    #use node
    rclpy.spin(listenerNode)

    #destroy node
    listenerNode.destroy_node()

    rclpy.shutdown()


if __name__ == "__main__":
    main()