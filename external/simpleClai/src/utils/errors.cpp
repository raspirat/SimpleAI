#include "errors.h"

#include <QSettings>
#include <QStringList>
#include <QCoreApplication>


const char* error::environment::DATASETS_PATH_Error::what() const noexcept
{
    return "\033[36m[ALERT]: Could not find SA_DATASET_PATH. Is it deleted? Please set it to a Path where your DATASET will be stored.\033[0m";
}

const char* error::environment::PROFILES_PATH_Error::what() const noexcept
{
    return "\033[36m[ALERT]: Could not find SA_DATASET_PATH. Is it deleted? Please set it to a Path where your DATASET will be stored.\033[0m";
}

const char* error::compatibility::LabelExtentionError::what() const noexcept
{
    QSettings settings("/etc/" + QCoreApplication::applicationName() +"/config/config.ini", QSettings::IniFormat);
    QStringList formats = settings.value("dataset/supported_labeling_formats").toStringList();
    return ("\033[31m[ERROR] <FATAL>: No files found! Probably wrong path or unsupported labeling format. Currently supported formats: " + formats.join(", ") + "\033[0m").toUtf8().constData();
}
const char* error::compatibility::ImageExtentionError::what() const noexcept
{
    QSettings settings("/etc/" + QCoreApplication::applicationName() + "/config/config.ini", QSettings::IniFormat);
    QStringList formats = settings.value("dataset/supported_img_formats").toStringList();
    return ("\033[31m[ERROR] <FATAL>: No files found! Probably wrong path or unsupported image format. Currently supported formats: " + formats.join(", ") + "\033[0m").toUtf8().constData();
}
const char* error::compatibility::IncompatibleLabelsError::what() const noexcept
{
    return "\033[31m[ERROR] <FATAL>: The labels of the dataset provided are not compatible with the labels required for the profile given.";
}


const char* error::name::DatasetNameError::what() const noexcept
{
    return "\033[31m[ERROR] <FATAL>: Dataset has the same name as an other one!\033[0m";
}
const char* error::name::ModelNameError::what() const noexcept
{
    return "\033[31m[ERROR] <FATAL>: Model has the same name as an other one!\033[0m";
}
const char* error::name::ProfileNameError::what() const noexcept
{
    return "\033[31m[ERROR] <FATAL>: Profile has the same name as an other one!\033[0m";
}
const char* error::name::ProjectNameError::what() const noexcept
{
    return "\033[31m[ERROR] <FATAL>: Project has the same name as an other one!\033[0m";
}


const char* error::existence::NoSuchModelError::what() const noexcept
{
    return "\033[31m[ERROR] <FATAL>: There is no such Model available!\033[0m";
}
const char* error::existence::NoSuchProjectError::what() const noexcept
{
    return "\033[31m[ERROR] <FATAL>: There is no such Project available!\033[0m";
}
const char* error::existence::NoSuchScopeError::what() const noexcept
{
    return "\033[31m[ERROR] <FATAL>: There is no such scope available!\033[0m";
}
const char* error::existence::NoSuchFrameworkError::what() const noexcept
{
    return "\033[31m[ERROR] <FATAL>: There is no such Framework available!\033[0m";
}
const char* error::existence::NoSuchDatasetError::what() const noexcept
{
    return "\033[31m[ERROR] <FATAL>: There is no such Dataset available!\033[0m";
}
const char* error::existence::NoSuchProfileError::what() const noexcept
{
    return "\033[31m[ERROR] <FATAL>: There is no such Profile available!\033[0m";
}
