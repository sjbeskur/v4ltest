import cv2   #include opencv library functions in python

#Create an object to hold reference to camera video capturing
vidcap = cv2.VideoCapture(0)

#check if connection with camera is successfully
if vidcap.isOpened():
    ret, frame = vidcap.read()  #capture a frame from live video

    gray = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)
    #check whether frame is successfully captured
    if ret:
        # continue to display window until 'q' is pressed
        while(True):

            cv2.imshow("Frame",gray)   #show captured frame
            #press 'q' to break out of the loop
            if cv2.waitKey(1) & 0xFF == ord('q'):
                break

                    # writing the extracted images 
        cv2.imwrite("frame_capture.png", gray) 
    #print error if frame capturing was unsuccessful
    else:
        print("Error : Failed to capture frame")

# print error if the connection with camera is unsuccessful
else:
    print("Cannot open camera")
    