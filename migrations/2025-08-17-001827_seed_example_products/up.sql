-- Only insert if table is empty (Postgres-specific, works for simple seeds)
DO $$
BEGIN
  IF NOT EXISTS (SELECT 1 FROM product) THEN
    INSERT INTO product
      (name, price, inventory, category, tags, keywords, thumbnail_url,
       gallery_urls, tagline, description, discount_percent, added_date, restock_date)
    VALUES
    ('Red Apple', 120, 100, 'Produce', 'Fruit,Healthy', 'red,apple,malus',
       'https://en.wikipedia.org/static/images/icons/wikipedia.png',
       'https://commons.wikimedia.org/wiki/File:Box_of_Marbles.jpg',
       'A crisp, tasty red apple!', 'Only the freshest...', 10.0, now(), NULL),
      
    ('Green Apple', 110, 130, 'Produce', 'Fruit,Healthy', 'green,apple,malus',
       'https://en.wikipedia.org/static/images/icons/wikipedia.png',
       'https://commons.wikimedia.org/wiki/File:Box_of_Marbles.jpg',
       'A crisp, tangy green apple!', 'Only the luigiest...', NULL, now(), NULL),

    ('Blue Apple', 160, 130, 'Produce', 'Fruit,Healthy', 'blue,apple,malus',
       'https://en.wikipedia.org/static/images/icons/wikipedia.png',
       'https://commons.wikimedia.org/wiki/File:Box_of_Marbles.jpg',
       'A crisp, off-putting blue apple!', 'Yes, they exist!', 33.0, now(), NULL),

    ('Grape', 4, 999, 'Produce', 'Fruit,Healthy', 'grapey',
       'https://en.wikipedia.org/static/images/icons/wikipedia.png',
       'https://commons.wikimedia.org/wiki/File:Box_of_Marbles.jpg',
       'Just one!', 'Actually, man, you can just take it.', NULL, now(), NULL),

    ('Kernberries', 420, 99, 'Produce', 'Fruit', 'kern,berry',
       'https://en.wikipedia.org/static/images/icons/wikipedia.png',
       'https://commons.wikimedia.org/wiki/File:Box_of_Marbles.jpg',
       'Why do they look like that?', 'Honestly, these do not sell very well...', NULL, now(), NULL),
      
    ('Coffee', 720, 34, 'Beverage', 'Caffeine', 'brewed,hot',
       'https://en.wikipedia.org/static/images/icons/wikipedia.png',
       'https://commons.wikimedia.org/wiki/File:Box_of_Marbles.jpg',
       'Burnt roast from elsewhere!', 'Only the coffeeiest...', NULL, now(), NULL),
      
    ('Tea', 400, 50, 'Beverage', 'Tea', 'brewed,cold',
       'https://en.wikipedia.org/static/images/icons/wikipedia.png',
       'https://commons.wikimedia.org/wiki/File:Box_of_Marbles.jpg',
       'Bagged!', 'Mostly unspilled!', 5.0, now(), NULL),
      
    ('Malk', 110, 7, 'Beverage', 'Dairy', 'cold',
       'https://en.wikipedia.org/static/images/icons/wikipedia.png',
       'https://commons.wikimedia.org/wiki/File:Box_of_Marbles.jpg',
       'Now with Vitamin R!', 'From the pastures of...', NULL, now(), NULL),

    ('Pink Drink', 99, 310, 'Beverage', 'Soft', 'pink,carbonated',
       'https://en.wikipedia.org/static/images/icons/wikipedia.png',
       'https://commons.wikimedia.org/wiki/File:Box_of_Marbles.jpg',
       'It is not green!', 'The machinations of this beverage are an enigma.', NULL, now(), NULL),

    ('Green Drink', 289, 218, 'Beverage', 'Soft', 'green,carbonated',
       'https://en.wikipedia.org/static/images/icons/wikipedia.png',
       'https://commons.wikimedia.org/wiki/File:Box_of_Marbles.jpg',
       'The bubbles hurt my nose!', 'It is not pink!', NULL, now(), NULL),
      
    ('Kernberry Pie', 12399, 8, 'Bakery', 'Pie', 'kern,berry',
       'https://en.wikipedia.org/static/images/icons/wikipedia.png',
       'https://commons.wikimedia.org/wiki/File:Box_of_Marbles.jpg',
       'For eating!', 'Loaded with luscious Kernberries...', NULL, now(), NULL),
      
    ('Bumble Pie', 3, 3, 'Bakery', 'Pie', 'bees',
       'https://en.wikipedia.org/static/images/icons/wikipedia.png',
       'https://commons.wikimedia.org/wiki/File:Box_of_Marbles.jpg',
       'Errrrm', 'It makes a buzzing noise?', NULL, now(), NULL),
      
    ('Rust Cookie', 399, 50, 'Bakery', 'Rust,Cookie', 'rusty',
       'https://en.wikipedia.org/static/images/icons/wikipedia.png',
       'https://commons.wikimedia.org/wiki/File:Box_of_Marbles.jpg',
       'Disgusting!', 'Some people like it.', 25.0, now(), NULL);
  END IF;
END$$;