#include "loginscreen.h"
#include "constants.h"
#include "networkmanager.h"
#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QFrame>

LoginScreen::LoginScreen(QWidget *parent)
    : QWidget(parent)
{
    this->setupConnections();
    // Set up main layout
    QVBoxLayout *mainLayout = new QVBoxLayout(this);
    mainLayout->setSpacing(20);
    mainLayout->setContentsMargins(40, 40, 40, 40);

    // Create header
    QLabel *headerLabel = new QLabel(Constants::PROJECT_NAME, this);
    QFont headerFont = headerLabel->font();
    headerFont.setPointSize(24);
    headerFont.setBold(true);
    headerLabel->setFont(headerFont);
    headerLabel->setAlignment(Qt::AlignCenter);

    QLabel *subtitleLabel = new QLabel("Login to your account", this);
    QFont subtitleFont = subtitleLabel->font();
    subtitleFont.setPointSize(14);
    subtitleLabel->setFont(subtitleFont);
    subtitleLabel->setAlignment(Qt::AlignCenter);

    // Add header to main layout
    mainLayout->addWidget(headerLabel);
    mainLayout->addWidget(subtitleLabel);
    mainLayout->addSpacing(20);

    // Create a centered form container
    QFrame *formContainer = new QFrame(this);
    formContainer->setFrameShape(QFrame::StyledPanel);
    formContainer->setMaximumWidth(400);
    formContainer->setSizePolicy(QSizePolicy::Preferred, QSizePolicy::Maximum);

    QVBoxLayout *containerLayout = new QVBoxLayout(formContainer);
    containerLayout->setSpacing(16);

    // Create form fields
    QLabel *usernameLabel = new QLabel("Username:", this);
    m_usernameEdit = new QLineEdit(this);
    m_usernameEdit->setPlaceholderText("Enter your username");

    QLabel *passwordLabel = new QLabel("Password:", this);
    m_passwordEdit = new QLineEdit(this);
    m_passwordEdit->setEchoMode(QLineEdit::Password);
    m_passwordEdit->setPlaceholderText("Enter your password");

    m_rememberCheckBox = new QCheckBox("Remember me", this);

    m_forgotPasswordLabel = new QLabel("<a href=\"#\">Forgot Password?</a>");
    m_forgotPasswordLabel->setTextFormat(Qt::RichText);
    m_forgotPasswordLabel->setTextInteractionFlags(Qt::TextBrowserInteraction);
    m_forgotPasswordLabel->setAlignment(Qt::AlignRight);

    // Add form fields to container layout
    containerLayout->addWidget(usernameLabel);
    containerLayout->addWidget(m_usernameEdit);
    containerLayout->addWidget(passwordLabel);
    containerLayout->addWidget(m_passwordEdit);

    // Add remember me and forgot password in a horizontal layout
    QHBoxLayout *checkLayout = new QHBoxLayout();
    checkLayout->addWidget(m_rememberCheckBox);
    checkLayout->addWidget(m_forgotPasswordLabel);
    containerLayout->addLayout(checkLayout);

    // Create login button
    m_loginButton = new QPushButton("Login", this);
    m_loginButton->setStyleSheet("background-color: #0a36d1; color: white; ");
    m_loginButton->setDefault(true);
    containerLayout->addWidget(m_loginButton);

    // Add separator
    QFrame *separator = new QFrame(this);
    separator->setFrameShape(QFrame::HLine);
    separator->setFrameShadow(QFrame::Sunken);
    containerLayout->addWidget(separator);

    // Create register section
    QLabel *noAccountLabel = new QLabel("Don't have an account?", this);
    noAccountLabel->setAlignment(Qt::AlignCenter);
    containerLayout->addWidget(noAccountLabel);

    m_registerButton = new QPushButton("Register", this);
    containerLayout->addWidget(m_registerButton);

    // Add container to main layout with horizontal centering
    QHBoxLayout *centeringLayout = new QHBoxLayout();
    centeringLayout->addStretch();
    centeringLayout->addWidget(formContainer);
    centeringLayout->addStretch();
    mainLayout->addLayout(centeringLayout);
    mainLayout->addStretch();

    // Add a footer with version information
    QLabel *versionLabel = new QLabel("Version " + Constants::VERSION, this);
    versionLabel->setAlignment(Qt::AlignCenter);
    mainLayout->addWidget(versionLabel);

    // Connect signals and slots
    connect(m_loginButton, &QPushButton::clicked, this, &LoginScreen::onLoginClicked);
    connect(m_forgotPasswordLabel, &QLabel::linkActivated, this, &LoginScreen::onForgotPasswordClicked);
    connect(m_registerButton, &QPushButton::clicked, this, &LoginScreen::onRegisterClicked);

    // Set tab order
    setTabOrder(m_usernameEdit, m_passwordEdit);
    setTabOrder(m_passwordEdit, m_rememberCheckBox);
    setTabOrder(m_rememberCheckBox, m_loginButton);
    setTabOrder(m_loginButton, m_registerButton);

    // Set initial focus
    m_usernameEdit->setFocus();
}
void LoginScreen::setupConnections() {

    qDebug() << "SETTING UP CONNECTIONS";
    connect(&NetworkManager::instance(), &NetworkManager::loginSuccess,
            this, &LoginScreen::onLoginSuccess);
    connect(&NetworkManager::instance(), &NetworkManager::loginFailure,
            this, &LoginScreen::onLoginFailure);
}

void LoginScreen::onLoginSuccess(const QJsonObject& responseData, const User& user) {
    // Handle successful login
    // QString token = responseData["token"].toString();
    // Store token, update UI, navigate to main screen, etc.
    User u = user;
    qDebug() << "LOGIN_SUCCESS. SIGNAL_EMIT";
    emit loginSuccessful(u);
}

void LoginScreen::onLoginFailure(const QString& errorMessage) {
    QMessageBox::critical(this, "Login Error", "Login failed. Username or password is incorrect.");
    qDebug() << "Login Failure..." << errorMessage;
}

void LoginScreen::onLoginClicked()
{
    QString username = m_usernameEdit->text();
    QString password = m_passwordEdit->text();

    if (username.isEmpty() || password.isEmpty()) {
        QMessageBox::warning(this, "Login Error", "Please enter both username and password to login.");
        return;
    }

    User user(username, password);
    NetworkManager::instance().login(user);

    // if (username == "admin" && password == "password") {
    //     if (m_rememberCheckBox->isChecked()) {
    //         saveCredentials(username);
    //     }

    //     // Create a user object and emit login successful signal
    //     User user;
    //     // Set user properties (assuming User class has these methods)
    //     // user.setUsername(username);

    //     emit loginSuccessful(user);
    // } else {
    //     QMessageBox::critical(this, "Login Failed", "Invalid username or password");
    // }
}

void LoginScreen::onForgotPasswordClicked()
{
    QMessageBox::information(this, "Reset Password", "Password reset functionality would be implemented here");
}

void LoginScreen::onRegisterClicked()
{
    emit registerRequested();
}

void LoginScreen::saveCredentials(const QString &username)
{
    qDebug("Remembering login for user: %s", qPrintable(username));
    // In a real implementation, you would store this securely
    // using QSettings or a more secure approach
}
