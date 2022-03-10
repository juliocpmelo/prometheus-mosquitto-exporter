#!/bin/bash
### BEGIN INIT INFO
# Provides:          prometheus-mosquitto-exporter
# Required-Start:    $remote_fs $syslog
# Required-Stop:
# Default-Start:
# Default-Stop:
# Short-Description: your description here
### END INIT INFO

# Source options file
. /etc/prometheus-mosquitto-exporter/service-env

#the service name
SNAME=prometheus-mosquitto-exporter

#the full path and name of the daemon program
PROG=/usr/sbin/$SNAME
PIDFILE=/var/run/$SNAME.pid


# start function
start() {
    #check the daemon status first
    if [ -f $PIDFILE ]
    then
        echo "$SNAME is already started!"
        exit 0;
    else
        echo $"Starting $SNAME ..."
        if start-stop-daemon --start --quiet --oknodo --background  --make-pidfile --pidfile ${PIDFILE} --startas /bin/bash -- -c "exec $PROG $OPTIONS > /var/log/$SNAME.log 2>&1" ; then
          echo $"Service $SNAME started"
        else
          echo $"$SNAME failed to start"
        fi
        exit 0;
    fi
}

#stop function
stop() {
  echo "Stopping $SNAME ..."
  if start-stop-daemon --stop --quiet --oknodo --pidfile ${PIDFILE}; then
    rm -f ${PIDFILE}
  else
    echo "Service $SNAME failed to stop"
  fi
}

case "$1" in
start)
  start
  ;;
stop)
  stop
  ;;
reload|restart)
  stop
  start
  ;;
status)
  if [ -f $PIDFILE ]; then
      cat /var/log/$SNAME.log
  else
    echo "$SNAME is stoped."
  fi
  ;;
*)
  echo $"\nUsage: $0 {start|stop|restart|status}"
  exit 1
esac