#!/bin/bash

dtTestVersion=1
dtName="test-dataset"

for dir in $HOME/.sclai/datasets/*/;
do
    if [[ "$dtName$dtTestVersion" == "$(basename $dir)" ]];
    then
        ((++dtTestVersion))
    fi
done

fullDtName=$dtName$dtTestVersion


echo "--------------------| starting test 1 |--------------------"

if sclai create dataset -n $fullDtName -l dataset-test/labelmap.pbtxt -a dataset-test/train -m dataset-test/train;
then
    echo -e "Created dataset succesfully! -> \033[32mtest 1 successful\033[0m"
    test1="\033[32msucceded\033[0m"

else
    echo -e "Failed to create dataset! -> \033[31mtest 1 failed\033[0m"
    test1="\033[31mfailed\033[0m"
fi

echo "--------------------| test 1 completed |--------------------"



echo "--------------------| starting test 2 |--------------------"

if sclai delete -d $fullDtName;
then
    echo -e "Deleted dataset succesfully! -> \033[32mtest 2 successful\033[0m"
    test2="\033[32msucceded\033[0m"

else
    echo -e "Failed to delete dataset! -> \033[31mtest 2 failed\033[0m"
    test2="\033[31mfailed\033[0m"
fi

echo "--------------------| test 2 completed |--------------------"

echo "summary:"
echo -e "test1: |$test1|"
echo -e "test2: |$test2|"

