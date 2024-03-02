#ifndef ERRORS_H
#define ERRORS_H

#include <exception>

namespace error
{

class GeneralError : public std::exception
{
public:
    virtual const char* what() const noexcept = 0;
};

namespace environment
{
    class EnvrionmentError : public GeneralError
    {
    public:
        virtual const char* what() const noexcept = 0;
    };

    class DATASETS_PATH_Error: public EnvrionmentError
    {
    public:
        const char* what() const noexcept override;
    };
    class PROFILES_PATH_Error : public EnvrionmentError
    {
    public:
        const char* what() const noexcept override;
    };
}

namespace compatibility
{
    class CompatibilityError : public GeneralError
    {
    public:
        virtual const char* what() const noexcept = 0;
    };

    class ImageExtentionError : public CompatibilityError
    {
    public:
        const char* what() const noexcept override;
    };
    class LabelExtentionError : public CompatibilityError
    {
    public:
        const char* what() const noexcept override;
    };

    class IncompatibleLabelsError : public CompatibilityError
    {
    public:
        const char* what() const noexcept override;
    };
}

namespace name
{
    class NameError : public GeneralError
    {
    public:
        virtual const char* what() const noexcept = 0;
    };

    class DatasetNameError : public NameError
    {
    public:
        const char* what() const noexcept override;
    };
    class ModelNameError : public NameError
    {
    public:
        const char* what() const noexcept override;
    };
    class ProfileNameError : public NameError
    {
    public:
        const char* what() const noexcept override;
    };
    class ProjectNameError : public NameError
    {
    public:
        const char* what() const noexcept override;
    };
}

namespace existence
{
    class ExistenceError : public GeneralError
    {
    public:
        virtual const char* what() const noexcept = 0;
    };

    class NoSuchProjectError : public ExistenceError
    {
    public:
        const char* what() const noexcept override;
    };
    class NoSuchModelError : public ExistenceError
    {
    public:
        const char* what() const noexcept override;
    };
    class NoSuchScopeError : public ExistenceError
    {
    public:
        const char* what() const noexcept override;
    };
    class NoSuchFrameworkError : public ExistenceError
    {
    public:
        const char* what() const noexcept override;
    };
    class NoSuchProfileError : public ExistenceError
    {
    public:
        const char* what() const noexcept override;
    };
    class NoSuchDatasetError : public ExistenceError
    {
    public:
        const char* what() const noexcept override;
    };
}
}

#endif // ERRORS_H
