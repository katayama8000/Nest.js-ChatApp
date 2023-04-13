import {
  Body,
  ClassSerializerInterceptor,
  Controller,
  Get,
  HttpException,
  HttpStatus,
  Inject,
  Param,
  ParseIntPipe,
  Post,
  UseFilters,
  UseInterceptors,
  UsePipes,
  ValidationPipe,
} from '@nestjs/common';
import { type CreateUserDto } from 'src/users/dto/createUser.dto';
import { UserNotFoundException } from 'src/users/exceptions/UserNotFound.exception';
import { HttpExceptionFilter } from 'src/users/filters/HttpException.filter';
import { SerializedUser } from 'src/users/types';

@Controller('users')
export class UsersController {
  constructor(@Inject('USERS_SERVICE') private usersService) {}

  @Get('')
  getUsers() {
    return this.usersService.getUsers();
  }

  @UseInterceptors(ClassSerializerInterceptor)
  @Get('username/:username')
  getUsersByUsername(@Param('username') username: string) {
    const user = this.usersService.getUsersByUsername(username);
    if (user) return new SerializedUser(user);
    else throw new HttpException('User not found', HttpStatus.BAD_REQUEST);
  }

  @UseInterceptors(ClassSerializerInterceptor)
  @UseFilters(HttpExceptionFilter)
  @Get('id/:id')
  getUserById(@Param('id', ParseIntPipe) id: number) {
    console.log(typeof id);
    const user = this.usersService.getUserById(id);
    console.log(user);
    if (user) {
      console.log('user: ', user);
      return new SerializedUser(user);
    } else {
      console.log('user not found');
      throw new UserNotFoundException('User not found', 400);
    }
  }

  @Post('create')
  @UsePipes(ValidationPipe)
  createUser(@Body() CreateUserDto: CreateUserDto) {
    return this.usersService.createUser(CreateUserDto);
  }
}
