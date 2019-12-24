awk 'BEGIN {
        while("df -hP " | getline) {
                if ( $NF == "/" ) {
                        printf "{\"totalDisk\": \"%d\",\"usedDisk\":\"%d\",", $2,$3
                }
        }
        while( getline  < "/proc/loadavg" ) {
                printf "\"cpu\":\"%.2f\",", $(NF-2)
        }

        while( "free -m"| getline) {
                if( $0 ~ /Mem:/) {
                printf "\"totalMemory\":\"%s\",\"usedMemory\":\"%s\", \"hostName\":\"Enetric\",\"os\":\"Linux 4.1.15\"}", $2,$3
                }
        }
}'
