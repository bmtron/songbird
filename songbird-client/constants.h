#ifndef CONSTANTS_H
#define CONSTANTS_H
#pragma once

#include <QString>

namespace Constants {
const QString PROJECT_NAME = "Songbird";
const QString VERSION = "0.0.1";
const int PASSWORD_MIN_LEN = 8;
const QString DEBUG_SERVER_ADDR = "http://localhost:3000";
const QString CREATE_USER_ENDPOINT = "/api/users/create";
const QString LOGIN_USER_ENDPOINT = "/api/login";

}
#endif // CONSTANTS_H
