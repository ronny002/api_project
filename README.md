  # Stackoverflow-like API application  
My version of the Stackoverflow-like API application for the [Let's Get Rusty](https://github.com/letsgetrusty/bootcamp/tree/master/4.%20Projects/2.%20API/Problem) Bootcamp.

Start postgres (req.: docker is installed and a container with postgres exists under the name stack-overflow-db):\
`sudo docker container start stack-overflow-db`\
tokio needs nightly:\
`cargo +nightly run`
