program example
  integer :: i, argc
  character(len=256) :: arg
  argc = command_argument_count()
  write(*,'(A)',advance='no') '[Fortran] args:'
  do i = 1, argc
    call get_command_argument(i, arg)
    write(*,'(1x,A)',advance='no') trim(arg)
  end do
  write(*,*)
end program example
