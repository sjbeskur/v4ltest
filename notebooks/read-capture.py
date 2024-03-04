import cv2

# Open the video file
video_path = 'video.mp4'
cap = cv2.VideoCapture(video_path)

# Check if the video file was opened successfully
if not cap.isOpened():
    print("Error: Could not open video file.")
    exit()

# Get video properties (frame width, height, and frames per second)
frame_width = int(cap.get(cv2.CAP_PROP_FRAME_WIDTH))
frame_height = int(cap.get(cv2.CAP_PROP_FRAME_HEIGHT))
fps = int(cap.get(cv2.CAP_PROP_FPS))

# Define the codec and create VideoWriter object
fourcc = cv2.VideoWriter_fourcc(*'XVID')  # Codec for saving video (XVID is a common choice)
output_path = 'output_video.avi'
out = cv2.VideoWriter(output_path, fourcc, fps, (frame_width, frame_height))

# Read and display frames from the video
while True:
    ret, frame = cap.read()

    # If frame is not read, it means end of the video
    if not ret:
        break

    # Display the frame
    cv2.imshow('Video', frame)

    # Write the frame to the output video
    out.write(frame)

    # Check for 'q' key press to exit
    if cv2.waitKey(25) & 0xFF == ord('q'):
        break

# Release the video capture and video writer objects
cap.release()
out.release()

# Close all OpenCV windows
cv2.destroyAllWindows()

print("Video saved to:", output_path)