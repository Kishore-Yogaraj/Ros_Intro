import rclpy
from rclpy.node import Node
from sensor_msgs.msg import Image
import cv2
from cv_bridge import CvBridge
import numpy as np

# This is a subscriber node that displays the images captured by the camera

class ImageSubscriber(Node): #Creating python class
    def __init__(self):
        super().__init__('image_subscriber')

        #Creation of the subscriber for the node
        #/camera/image_raw is the topic that is actually publishing the data from the camera 
        """
        When trying to figure out what to put in the callback function that's used as a
        parameter for the subscription method just think about what you want to happen once the
        message is received. In this case we want the node to begin detections once the image from the
        camera is received. So everytime an image is received, do the detection which is in the listener_callback
        method. 

        You can think of the constructor as almost the 'setup' for the node. The actual processing techniques of the node
        happens in your callback function.
        """
        self.subscription = self.create_subscription(Image, '/camera/image_raw', self.listener_callback, 10)  
        self.subscription  # prevent unused variable warning
        self.bridge = CvBridge() # Initializeing the class that allows us to use OpenCV

        # Load the stop sign classifier
        self.stop_sign_classifier = cv2.CascadeClassifier('/home/user/stop_data.xml')
        if self.stop_sign_classifier.empty():
            self.get_logger().warning("Failed to load stop sign classifier")
        
         # Load YOLOv3 model
        self.net = cv2.dnn.readNet("/home/user/yolov3.weights", "/home/user/yolov3.cfg") # The weights and configuration files are what were learned during training of the model
        
        #Layers of the neural network
        #These three lines are used to idenitify the name and index of the ouput layer which is used later for object detection otherwise it would simply pull outputs from every layer
        self.layer_names = self.net.getLayerNames()
        unconnected_layers = self.net.getUnconnectedOutLayers()
        self.output_layers = [self.layer_names[i - 1] for i in unconnected_layers]

        #Model has been trained to detect everything in this list
        with open("/home/user/coco.names", "r") as f:
            self.classes = [line.strip() for line in f.readlines()]
 
    # Call back function which executes everytime the subscriber receives a message
    # Function that actually displays the image
    def listener_callback(self, data):
        current_frame = self.bridge.imgmsg_to_cv2(data, desired_encoding='bgr8') # Converts images to cv2 format so that it can be processed and analyzed with cv2

        # Perform object detection
        blob = cv2.dnn.blobFromImage(current_frame, 0.00392, (416, 416), (0, 0, 0), True, crop=False) # Turns image to something YOLO can process
        self.net.setInput(blob) # Sets blob as the input
        """
        This line will proces the image through the neural netowrk and only pull out puts from the output
        layers instead of every layer.
        """
        outs = self.net.forward(self.output_layers) # Gets the input from specific output layers

         # Process detections so that it can be used to create our desired outputs
        class_ids = []
        confidences = []
        boxes = []
        height, width, channels = current_frame.shape
        for out in outs:
            for detection in out:
                scores = detection[5:]
                class_id = np.argmax(scores)
                confidence = scores[class_id]
                if confidence > 0.5:
                    # Object detected
                    center_x = int(detection[0] * width)
                    center_y = int(detection[1] * height)
                    w = int(detection[2] * width)
                    h = int(detection[3] * height)
                    x = int(center_x - w / 2)
                    y = int(center_y - h / 2)
                    boxes.append([x, y, w, h])
                    confidences.append(float(confidence))
                    class_ids.append(class_id)

        # Non-max suppression to remove overlapping bounding boxes
        indexes = cv2.dnn.NMSBoxes(boxes, confidences, 0.5, 0.4)
        for i in range(len(boxes)):
            if i in indexes:
                x, y, w, h = boxes[i]
                label = str(self.classes[class_ids[i]])
                confidence = confidences[i]
                color = (0, 255, 0)
                cv2.rectangle(current_frame, (x, y), (x + w, y + h), color, 2)
                cv2.putText(current_frame, f"{label} {confidence:.2f}", (x, y - 10), cv2.FONT_HERSHEY_SIMPLEX, 0.5, color, 2)

         # Convert to grayscale because colours aren't important for the detection
        gray = cv2.cvtColor(current_frame, cv2.COLOR_BGR2GRAY)

        # Detect stop signs
        if not self.stop_sign_classifier.empty():
            stop_signs = self.stop_sign_classifier.detectMultiScale(gray, 1.3, 5) # Does the detections and saves it in the variable 'stop_signs'

            # Draw bounding boxes around the stop signs
            for (x, y, w, h) in stop_signs:
                cv2.rectangle(current_frame, (x, y), (x + w, y + h), (0, 0, 255), 2)

        cv2.imshow("Camera Feed", current_frame) # Used to display data captured by camera
        cv2.waitKey(1)
 
def main(args=None):
    rclpy.init(args=args)
    image_subscriber = ImageSubscriber()
    rclpy.spin(image_subscriber)
    image_subscriber.destroy_node()
    rclpy.shutdown()
 
if __name__ == '__main__':
    main()