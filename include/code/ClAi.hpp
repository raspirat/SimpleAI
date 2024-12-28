#ifndef CLAI_H
#define CLAI_H

#include <QObject>
#include <QJsonObject>
#include "external/simpleClai/src/config/config.h"
#include "external/simpleClai/src/utils/tools.h"
#include "external/simpleClai/src/commands/project.h"
#include "external/simpleClai/src/commands/profile.h"
#include "external/simpleClai/src/commands/model.h"
#include "external/simpleClai/src/commands/dataset.h"

class ClAi : public QObject
{
    Q_OBJECT
public:
    ClAi();
public slots:
    static const QString getDatasetsConfigPath();
    static const QString getProfilesConfigPath();
    static const QString getProjectsConfigPath();
    static QJsonObject getDatasetsJson();
    static QJsonObject getProfilesJson();
    static QJsonObject getProjectsJson();
    static QJsonObject getModelsJson();
    static bool saveJson(const QJsonObject & object, const QString & filePath);

    int createProject(const QString&, const QString&, const QString&);
    int deleteProject(const QString&);

    int createProfile(const QString&, const QString&, const QString&);
    int deleteProfile(const QString&);

    int createModel(const QString&, const QString&, const QString&);
    int deleteModel(const QString&, const QString&);
    int trainModel(const QString&, const QString&, const QString&);

    int createDataset(const QString&, const QString&, const QString&);
    int deleteDataset(const QString&);
};

#endif // CLAI_H
