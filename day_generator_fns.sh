#!/usr/bin/env bash

generate_scaffolding () {
    day=$1
    year=$2
    template_file=$3

    day_no_leading_zero=$(echo $day | sed 's/^0*//')

    cat $template_file | sed -e "s/{YYYY}/$year/g" -e "s/{NN}/$1/g" -e "s/{N}/$day_no_leading_zero/" > src/bin/day$1.rs;
    mkdir -p inputs/day$day;
    touch inputs/day$day/{input.txt,sample.txt};

    today=$(TZ=America/New_York date +%d)
    current_year=$(TZ=America/New_York date +%Y)

    if [[ $day -le $today || $year -le $current_year ]]
    then
        curl -b "session=$(cat ../session.txt)" "https://adventofcode.com/$year/day/$day_no_leading_zero/input" > inputs/day$day/input.txt
    else
        echo "Cannot get input! It's not time yet!"
    fi
}


