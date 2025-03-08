#ifndef NETWORKMANAGER_H
#define NETWORKMANAGER_H
#include <QtNetwork/QNetworkAccessManager>
#include <QJsonObject>
#include <QJsonDocument>
#include <QByteArray>
#include <QNetworkReply>
#include "user.h"

class NetworkManager : public QObject
{
    Q_OBJECT
public:
    static NetworkManager& instance();
    QNetworkAccessManager* manager();
    void registerUser(User &user);
private:
    NetworkManager(QObject *parent = nullptr);
    QNetworkAccessManager *network_manager;
};

#endif // NETWORKMANAGER_H
