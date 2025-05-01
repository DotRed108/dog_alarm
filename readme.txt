In windows task scheduler the optional "start in" setting must be set to the project folder like so
C:\Users\dotred\Projects\dog_alarm


TO KEEP BLUETOOTH AUTOMATICALLY CONNECtED IN DUAL BOOT SETUP 

START BY REMOVING BLUETOOTH DEVICE THEN
-sudo apt install chntpw;
-sudo pip install bt-dualboot;
THEN MOUNT WINDOWS WITH FOLLOWING COMMAND
-bt-dualboot --win /foo_dir/windows_location -l # Replace 
THEN SYNC BLUETOOTH
-sudo bt-dualboot -b --sync-all

references
https://github.com/x2es/bt-dualboot
https://askubuntu.com/questions/1532415/how-to-reconnect-bluetooth-headphones-after-dual-boot-to-windows-11