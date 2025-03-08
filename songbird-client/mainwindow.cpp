#include "mainwindow.h"
#include "./ui_mainwindow.h"
#include "constants.h"
#include <QVBoxLayout>
#include <QLabel>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
    , ui(new Ui::MainWindow)
{
    ui->setupUi(this);

    // Set the window title
    setWindowTitle(Constants::PROJECT_NAME);

    // Create a stacked widget to manage our different screens
    m_stackedWidget = new QStackedWidget(this);
    setCentralWidget(m_stackedWidget);

    // Create the login screen
    m_loginScreen = new LoginScreen(this);

    // Create the registration screen
    m_registrationScreen = new RegistrationScreen(this);

    // Create the main interface (placeholder for now)
    m_mainInterface = new QWidget(this);
    setupMainInterface();

    // Add screens to stacked widget
    m_stackedWidget->addWidget(m_loginScreen);
    m_stackedWidget->addWidget(m_registrationScreen);
    m_stackedWidget->addWidget(m_mainInterface);

    // Connect signals and slots
    connect(m_loginScreen, &LoginScreen::registerRequested,
            this, &MainWindow::showRegistrationScreen);

    connect(m_loginScreen, &LoginScreen::loginSuccessful,
            this, &MainWindow::showMainInterface);

    connect(m_registrationScreen, &RegistrationScreen::registrationCancelled,
            this, &MainWindow::showLoginScreen);

    connect(m_registrationScreen, &RegistrationScreen::registrationCompleted,
            this, &MainWindow::showMainInterface);

    // Start with the login screen
    showLoginScreen();

    // Set a reasonable minimum size for the window
    setMinimumSize(800, 600);
}

MainWindow::~MainWindow()
{
    delete ui;
}

void MainWindow::showLoginScreen()
{
    m_stackedWidget->setCurrentWidget(m_loginScreen);
}

void MainWindow::showRegistrationScreen()
{
    m_stackedWidget->setCurrentWidget(m_registrationScreen);
}

void MainWindow::showMainInterface(User user)
{
    // Here you would typically update the UI based on the user
    // For example, displaying the user's name, profile picture, etc.

    // Switch to the main interface screen
    m_stackedWidget->setCurrentWidget(m_mainInterface);
}

void MainWindow::setupMainInterface()
{
    // This is a placeholder for your actual chat interface
    // You would replace this with your real implementation

    QVBoxLayout *layout = new QVBoxLayout(m_mainInterface);

    QLabel *welcomeLabel = new QLabel("Welcome to " + Constants::PROJECT_NAME + "!", m_mainInterface);
    QFont font = welcomeLabel->font();
    font.setPointSize(24);
    welcomeLabel->setFont(font);
    welcomeLabel->setAlignment(Qt::AlignCenter);

    QLabel *infoLabel = new QLabel("Main interface coming soon...", m_mainInterface);
    infoLabel->setAlignment(Qt::AlignCenter);

    layout->addStretch();
    layout->addWidget(welcomeLabel);
    layout->addWidget(infoLabel);
    layout->addStretch();
}
