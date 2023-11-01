export enum Unit {
    Miles,
    Kilometres,
    NauticalMiles
}

export function distance(lat1: number, lon1: number, lat2: number, lon2: number, unit: Unit = Unit.Miles) {
    if ((lat1 == lat2) && (lon1 == lon2)) return 0;

    const radlat1 = Math.PI * lat1/180;
    const radlat2 = Math.PI * lat2/180;
    const theta = lon1 - lon2;
    const radtheta = Math.PI * theta/180;
    let dist = Math.sin(radlat1) * Math.sin(radlat2) + Math.cos(radlat1) * Math.cos(radlat2) * Math.cos(radtheta);
    dist = Math.min(dist, 1);
    
    dist = Math.acos(dist);
    dist = dist * 180 / Math.PI;

    switch (unit) {
        case Unit.Kilometres:
            dist *= 1.609344;
            break;
        case Unit.NauticalMiles:
            dist *= 0.8684;
            break;
        default:
            dist *= 60 * 1.1515;
    }

    return dist;
}

