@echo off
setlocal

goto :case_%1 2>nul || goto :case_default

:case_server
    echo Running oxide Server
    cd server/
    cargo run
    goto :end_switch

:case_client
    echo Running oxide Client
    cd client/
    cargo run
    goto :end_switch

:case_default
    echo Invalid argument "%1"!
    goto :end_switch

:end_switch
echo Exiting oxide...
endlocal