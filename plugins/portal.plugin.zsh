#!/usr/bin/env zsh

export PORTAL_DEBUG=false
export PORTAL_DIR=/usr/local/lib/portal
export PORTAL_LOG=$PORTAL_DIR/portal.log

function z() {
    # p is used for 'print' in zsh, so using z instead
    result=`portal jump $1`
    if [[ "$result" =~ "Not found" ]]; then
      echo $result
    else
      cd $result
    fi
}

function track() {
    if [[ PORTAL_DEBUG == true ]]; then
	echo "Debugging Portal Tracker"
        portal db track $pwd
    else
        mkdir -p $PORTAL_DIR
        touch $PORTAL_LOG
        portal db track $pwd >> $PORTAL_LOG
        echo "-----------------------------------------------------------" >> $PORTAL_LOG
    fi
}

# Hook cd command to run tracker every change of dir
autoload -U add-zsh-hook
add-zsh-hook -Uz chpwd (){
    track
}
