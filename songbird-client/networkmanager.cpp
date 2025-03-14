#include "networkmanager.h"
#include "constants.h"

NetworkManager& NetworkManager::instance() {
    static NetworkManager instance;
    return instance;
}

NetworkManager::NetworkManager(QObject *parent) : QObject(parent) {
   network_manager = new QNetworkAccessManager(this);
}

QNetworkAccessManager* NetworkManager::manager() {
    return network_manager;
}

void NetworkManager::registerUser(User &user) {
    QUrl url("http://localhost:3000/api/users/create");

    QNetworkRequest request(url);

    request.setHeader(QNetworkRequest::ContentTypeHeader, "application/json");

    QJsonObject jsonObject;

    jsonObject["username"] = user.username;
    jsonObject["password"] = user.password;
    jsonObject["email"] = user.email;
    jsonObject["avatar_url"] = QJsonValue::Null;

    QJsonDocument jsonDoc(jsonObject);

    QByteArray jsonData = jsonDoc.toJson();
    QNetworkReply* reply = network_manager->post(request, jsonData);

    connect(reply, &QNetworkReply::finished, [this, reply]() {
        if (reply->error() == QNetworkReply::NoError) {
            QByteArray responseData = reply->readAll();
            QJsonDocument responseDoc = QJsonDocument::fromJson(responseData);
            QJsonObject responseObj = responseDoc.object();

            qDebug() << "Registration successful:" << responseObj["message"].toString();
        } else {
            qDebug() << "Registration error:" << reply->errorString();
        }

        reply->deleteLater();
    });
}

void NetworkManager::login(User &user) {

    QUrl url(Constants::DEBUG_SERVER_ADDR + Constants::LOGIN_USER_ENDPOINT);
    QNetworkRequest request(url);
    request.setHeader(QNetworkRequest::ContentTypeHeader, "application/json");

    QJsonObject jsonObject;

    jsonObject["username"] = user.username;
    jsonObject["password"] = user.password;

    QJsonDocument jsonDoc(jsonObject);
    QByteArray jsonData = jsonDoc.toJson();
    QNetworkReply* reply = network_manager->post(request, jsonData);

    connect(reply, &QNetworkReply::finished, this, [this, reply, user]() {
        if (reply->error() == QNetworkReply::NoError) {
            QByteArray responseData = reply->readAll();
            QJsonDocument responseDoc = QJsonDocument::fromJson(responseData);
            QJsonObject responseObj = responseDoc.object();

            qDebug() << "Login successful:" << responseObj["message"].toString();

            emit loginSuccess(responseObj, user);
        } else {
            qDebug() << "Login error:" << reply->errorString();
            emit loginFailure(reply->errorString());
        }
        reply->deleteLater();
    });
}
