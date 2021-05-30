let b = b2d.add_shape("jaman");
    b.add_animation( 2,
        5,
        67,
        7,
        "width");
    let c = b2d.add_shape("zeda");
    c.add_animation( 2,
        5,
        67,
        7,
        "color");

    //let name_of_shape = c.attributes.get_name();   
    println!("Get Shape Test :: {:?}",b2d.get_shape("zeda"));    
    let jj = b2d.get_shape("zeda").unwrap();
    
    let ccc = jj.attributes.get_bounding_rectangle_color();
    println!("color :: {:?}",ccc);    
    
    ////======
    let t  = b2d.get_shape("zeda")
    .unwrap().attributes.get_bounding_rectangle_color();
    println!("color :: {:?}",t);    
    
    b2d.draw();
    // b2d.shapes
