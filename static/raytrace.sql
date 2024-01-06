CREATE TEMPORARY TABLE constants (
    name TEXT PRIMARY KEY,
    value REAL
);

CREATE TEMPORARY TABLE vector3 (
    id TEXT PRIMARY KEY,
    x REAL,
    y REAL,
    z REAL
);

-- Image
INSERT INTO constants (name, value) VALUES ("aspect ratio", 16.0/9.0);
INSERT INTO constants (name, value) VALUES ("width", 400);
INSERT INTO constants (name, value) VALUES ("height", CAST((SELECT value FROM constants WHERE name = "width") / (SELECT value FROM constants WHERE name = "aspect ratio") AS INTEGER));

--Camera
INSERT INTO constants (name, value) VALUES ("viewport height", 2.0);
INSERT INTO constants (name, value) VALUES ("viewport width", (SELECT value FROM constants WHERE name = "aspect ratio") * (SELECT value FROM constants WHERE name = "viewport height"));
INSERT INTO constants (name, value) VALUES ("focal length", 1.0);

INSERT INTO vector3 (id, x, y, z) VALUES ("origin", 0.0, 0.0, 0.0);
INSERT INTO vector3 (id, x, y, z) VALUES ("horizontal", (SELECT value FROM constants WHERE name = "viewport width"), 0.0, 0.0);
INSERT INTO vector3 (id, x, y, z) VALUES ("vertical", 0.0, (SELECT value FROM constants WHERE name = "viewport height"), 0.0);
INSERT INTO vector3 (id, x, y, z) VALUES ("lower left corner",
	(SELECT x FROM vector3 WHERE id = "origin") - (SELECT x FROM vector3 WHERE id = "horizontal") / 2.0 - (SELECT x FROM vector3 WHERE id = "vertical") / 2.0,
	(SELECT y FROM vector3 WHERE id = "origin") - (SELECT y FROM vector3 WHERE id = "horizontal") / 2.0 - (SELECT y FROM vector3 WHERE id = "vertical") / 2.0,
	(SELECT z FROM vector3 WHERE id = "origin") - (SELECT z FROM vector3 WHERE id = "horizontal") / 2.0 - (SELECT z FROM vector3 WHERE id = "vertical") / 2.0 - (SELECT value FROM constants WHERE name = "focal length")
);

CREATE TEMPORARY TABLE sphere (
	id TEXT PRIMARY KEY,
	x REAL,
	y REAL,
	z REAL,
	radius REAL
);
INSERT INTO sphere (id, x, y, z, radius) VALUES ("big", 0.0, 0.0, -1.0, 0.5);

SELECT "P3";
SELECT PRINTF("%i %i",
    (SELECT value FROM constants WHERE name = "width"),
    (SELECT value FROM constants WHERE name = "height"));
SELECT "255";

.headers off
.mode tabs
WITH RECURSIVE numbers(x, y) AS (
	SELECT 0, 0
	UNION ALL
	SELECT
	   	(x + 1) % (SELECT value FROM constants WHERE name = "width"),
		y + CASE WHEN x = (SELECT value FROM constants WHERE name = "width") - 1 THEN 1 ELSE 0 END
	FROM numbers 
	WHERE y < (SELECT value FROM constants WHERE name = "height")
),
uv(x, y, u, v) AS (
	SELECT
		x, y,
		CAST(x AS REAL) / ((SELECT value FROM constants WHERE name = "width") - 1),
		1.0 - CAST(y AS REAL) / ((SELECT value FROM constants WHERE name = "height") - 1)
	FROM numbers
),
rays(x, y, ox, oy, oz, dx, dy, dz) AS (
	SELECT
		x, y,
		(SELECT x FROM vector3 WHERE id = "origin"),
		(SELECT y FROM vector3 WHERE id = "origin"),
		(SELECT z FROM vector3 WHERE id = "origin"),
		(SELECT x FROM vector3 WHERE id = "lower left corner") + u * (SELECT x FROM vector3 WHERE id = "horizontal") + v * (SELECT x FROM vector3 WHERE id = "vertical") - (SELECT x FROM vector3 WHERE id = "origin"),
		(SELECT y FROM vector3 WHERE id = "lower left corner") + u * (SELECT y FROM vector3 WHERE id = "horizontal") + v * (SELECT y FROM vector3 WHERE id = "vertical") - (SELECT y FROM vector3 WHERE id = "origin"),
		(SELECT z FROM vector3 WHERE id = "lower left corner") + u * (SELECT z FROM vector3 WHERE id = "horizontal") + v * (SELECT z FROM vector3 WHERE id = "vertical") - (SELECT z FROM vector3 WHERE id = "origin")
	FROM uv
),
rays_unit(x, y, ox, oy, oz, dx, dy, dz, ndx, ndy, ndz) AS (
	SELECT
		x, y,
		ox, oy, oz,
		dx, dy, dz,
		dx / SQRT(POW(dx, 2) + POW(dy, 2) + POW(dz, 2)),
		dy / SQRT(POW(dx, 2) + POW(dy, 2) + POW(dz, 2)),
		dz / SQRT(POW(dx, 2) + POW(dy, 2) + POW(dz, 2))
	FROM rays
),
hit_raw AS (
	SELECT
		x, y,
		ox, oy, oz,
		dx, dy, dz,
		ndx, ndy, ndz,
	
		-- oc is (ox - cx, oy - cy, oz - cz)
		-- a is (POW(dx, 2) + POW(dy, 2) + POW(dz, 2))
		-- b is 2.0 * (ocx * dx + ocy * dy + ocz * dz)
		-- c is (POW(ocx, 2) + POW(ocy, 2) + POW(ocz, 2)) - r^2
		-- d is POW(b, 2) - 4.0 * a * c
		-- h is d > 0

		(POW(dx, 2) + POW(dy, 2) + POW(dz, 2)) as ha,
		(2.0 * ((ox - (SELECT x FROM sphere WHERE id = "big")) * dx + (oy - (SELECT y FROM sphere WHERE id = "big")) * dy + (oz - (SELECT z FROM sphere WHERE id = "big")) * dz)) as hb,
		((POW((ox - (SELECT x FROM sphere WHERE id = "big")), 2) + POW((oy - (SELECT y FROM sphere WHERE id = "big")), 2) + POW((oz - (SELECT z FROM sphere WHERE id = "big")), 2)) - POW((SELECT radius FROM sphere WHERE id = "big"), 2)) as hc
	FROM rays_unit
),
hits AS (
	SELECT
		x, y,
		ox, oy, oz,
		dx, dy, dz,
		ndx, ndy, ndz,
		ha, hb, hc,

		CASE WHEN POW(hb, 2) - 4.0 * ha * hc < 0 THEN 
			-1.0 
		ELSE 
			((-1.0 * hb) - SQRT(POW(hb, 2) - 4.0 * ha * hc)) / (2.0 * ha)
		END as h
	FROM hit_raw
),
colours AS (
	SELECT
		x, y, h,

		CASE WHEN h > 0.0 THEN
			-- 0.5 * ((ox + h * dx) + 1)
			0.5 * ((dx / SQRT(POW(ox + h * dx, 2) + POW(oy + h * dy, 2) + (POW(oz + h * dz, 2)))) + 1)
		ELSE 
			((1.0 - 0.5 * (ndy + 1.0)) + 0.5 * (ndy + 1.0) * 0.5)
		END as r,

		CASE WHEN h > 0.0 THEN
			-- 0.5 * ((oy + h * dy) + 1)
			0.5 * ((dy / SQRT(POW(ox + h * dx, 2) + POW(oy + h * dy, 2) + (POW(oz + h * dz, 2)))) + 1)
		ELSE 
			((1.0 - 0.5 * (ndy + 1.0)) + 0.5 * (ndy + 1.0) * 0.7)
		END as g,

		CASE WHEN h > 0.0 THEN
			-- 0.5 * ((oz + h * dz) + 1) - 1
			0.5 * (dz / SQRT(POW(ox + h * dx, 2) + POW(oy + h * dy, 2) + (POW(oz + h * dz, 2))))
		ELSE 
			1.0
		END as b
	FROM hits
),
final(x, y, r, g, b, u, v, ndx, ndy, ndz, h) AS (
    SELECT 
        c.x, c.y,
        c.r, c.g, c.b,
        u.u, u.v,
		r.ndx, r.ndy, r.ndz,
		c.h
    FROM colours c
    JOIN uv u ON c.x = u.x AND c.y = u.y
	JOIN rays_unit r ON c.x = r.x AND c.y = r.y
)
SELECT PRINTF(
	"%3s %3s %3s      # %3f %3f      %3f %3f %3f 	%s",
	CAST(r * 255 AS INT),
	CAST(g * 255 AS INT),
	CAST(b * 255 AS INT),
	u, v,
	ndx, ndy, ndz,
	h
)
FROM final
WHERE y < (SELECT value FROM constants WHERE name = "height");


-- .headers off
-- .mode tabs
-- WITH RECURSIVE numbers(x, y) AS (
-- 	SELECT 0, 0
-- 	UNION ALL
-- 	SELECT 
-- 	   	(x + 1) % (SELECT value FROM constants WHERE name = "width"),
-- 		y + CASE WHEN x = (SELECT value FROM constants WHERE name = "width") - 1 THEN 1 ELSE 0 END
-- 	FROM numbers 
-- 	WHERE y < (SELECT value FROM constants WHERE name = "height")
-- ),
-- rays AS (
-- 	SELECT 
-- 		x, y, 
-- 		(x - (SELECT value FROM constants WHERE name = "width") / 2.0) / (SELECT value FROM constants WHERE name = "width") * 2.0,
-- 		(y - (SELECT value FROM constants WHERE name = "height") / 2.0) / (SELECT value FROM constants WHERE name = "height") * 2.0,
-- 		-1.0
-- 	FROM numbers
-- ),
-- spheres AS (
-- 	SELECT 
-- 		x, y, z, radius, material
-- 	FROM spheres
-- ),
-- intersections AS (
-- 	SELECT 
-- 		r.x, r.y, r.z, r.radius, r.material, 
-- 		s.x, s.y, s.z, s.radius, s.material,
-- 		(r.x - s.x) * (r.x - s.x) + (r.y - s.y) * (r.y - s.y) + (r.z - s.z) * (r.z - s.z) - s.radius * s.radius AS distance
-- 	FROM rays r
-- 	JOIN spheres s
-- 	ON 1 = 1
-- ),
-- closest AS (
-- 	SELECT 
-- 		x, y, z, radius, material, 
-- 		MIN(distance) AS distance
-- 	FROM intersections
-- 	GROUP BY x, y, z, radius, material
-- ),
-- colors AS (
-- 	SELECT 
-- 		x, y, z, radius, material, distance,
-- 		m.r * 255 AS r, m.g * 255 AS g, m.b * 255 AS b
-- 	FROM closest c
-- 	JOIN materials m
-- 	ON c.material = m.name
-- )
-- SELECT PRINTF("%3s %3s %3s # %3i %3i", r, g, b, x, y) FROM colors WHERE y < (SELECT value FROM constants WHERE name = "height");
