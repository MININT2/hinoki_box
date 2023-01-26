//io functions

mod bxfs;
//concerning the io instruction behavior on the bytecode side:
//make this set io flags: ok, pending, err
//OK means the operation completed and is ready for more
//PEND means an io operation is taking place, sets OK to 0, sets to 1 if successful, throws ERR
//ERR means the operation couldn't complete, sets OK to 0, sets PEND to 0