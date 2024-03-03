#ifndef DATASET_H
#define DATASET_H

#include <QString>

namespace dataset
{
int createDataset
    (
    const QString& name,
    const QString& labelmapPath,
    const QString& dataPath,
    const QString& labelsPath
    );
int deleteDataset
    (
    const QString& name,
    bool confirmationDialog = true
    );
void list
    (
    );
}

#endif // DATASET_H
