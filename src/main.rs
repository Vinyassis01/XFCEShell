use gtk4::prelude::*;
// Importamos o gio de dentro do gtk4 para evitar erros de dependência
use gtk4::{
    gio, Application, ApplicationWindow, SearchEntry, Label, Box, 
    Orientation, FlowBox, SelectionMode, Align, Image, ScrolledWindow, Button
};
use gtk4::Overlay;
use gtk4::{CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION};
use gtk4::gdk::Display; 

fn main() {
    let app = Application::builder()
        .application_id("com.exemplo.gnomeshell-clone")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn create_workspace_preview(id: i32) -> gtk4::Button {
    // 1. Container do Card (Box) - Centralização horizontal e vertical
    let workspace_card = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .spacing(10)
        .width_request(200)
        .halign(gtk4::Align::Center) // Centraliza o Box no espaço do botão
        .valign(gtk4::Align::Center) // Centraliza o Box verticalmente
        .css_classes(vec!["workspace-card".to_string()])
        .build();

    // 2. Placeholder/Miniatura centralizada
    let thumbnail = gtk4::Box::builder()
        .width_request(200)
        .height_request(120)
        .halign(gtk4::Align::Center)
        .build();

        // Definimos um NOME DE OBJETO para referenciar no CSS
        thumbnail.set_widget_name(&format!("thumb-ws-{}", id));
        thumbnail.add_css_class("window-thumbnail");

    workspace_card.append(&thumbnail);

    gtk4::Button::builder()
        .child(&workspace_card)
        .has_frame(false)
        .build()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("XFCE4Shell")
        .fullscreened(true)
        .build();

    // --- CONTAINER PRINCIPAL DOS WORKSPACES (Vertical) ---
    let workspaces_container = Box::new(Orientation::Vertical, 15);
    workspaces_container.set_halign(Align::Center);
    workspaces_container.set_margin_bottom(20);
    
    // Primeira linha: 3 miniaturas
    let row_top = Box::new(Orientation::Horizontal, 15);
    row_top.set_halign(Align::Center);
    
    // Segunda linha: 2 miniaturas
    let row_bottom = Box::new(Orientation::Horizontal, 15);
    row_bottom.set_halign(Align::Center);
    
    // Criamos as 5 miniaturas
    for i in 1..=5 {
        let ws_button = create_workspace_preview(i);
        
        // Ação para trocar de workspace
        let ws_index = i - 1;
        ws_button.connect_clicked(move |_| {
            let _ = std::process::Command::new("wmctrl")
                .args(["-s", &ws_index.to_string()])
                .spawn();
        });
    
        // Distribui os botões entre as linhas
        if i <= 3 {
            row_top.append(&ws_button);
    } else {
        row_bottom.append(&ws_button);
        }
    }
    
    // Monta a estrutura: Adiciona as duas linhas ao container vertical
    workspaces_container.append(&row_top);
    workspaces_container.append(&row_bottom);

    // Criamos um Overlay para permitir camadas (Fundo + Conteúdo)
    let overlay = Overlay::new();

    // --- IMAGEM DE FUNDO ---
    let background_image = gtk4::Picture::for_filename("ws1_thumb.jpg");
    background_image.set_content_fit(gtk4::ContentFit::Cover);
    overlay.set_child(Some(&background_image));

    // Dentro de build_ui(app: &Application)
    let provider = CssProvider::new();
   
    let css_data = include_str!("../style.css");
    provider.load_from_data(css_data);
    
    // Pega o display padrão e aplica o provedor de CSS
    if let Some(display) = Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    // --- CARREGAR IMAGEM DE FUNDO EMBUTIDA ---
    // include_bytes! lê o arquivo e gera um &[u8] estático
    let image_bytes = include_bytes!("../ws1_thumb.jpg");
    let bytes = gtk4::glib::Bytes::from_static(image_bytes);
   
    // Criamos a textura a partir dos bytes na memória
    let texture = gtk4::gdk::Texture::from_bytes(&bytes).expect("Erro ao carregar a textura dos bytes embutidos");
    let background_image = gtk4::Picture::for_paintable(&texture);
    
    background_image.set_content_fit(gtk4::ContentFit::Cover);
    overlay.set_child(Some(&background_image));


    let main_vbox = Box::new(Orientation::Vertical, 40);
    main_vbox.add_css_class("main-overlay");
    main_vbox.set_margin_top(10);
    main_vbox.set_margin_bottom(100);

    let search_bar = SearchEntry::builder()
        .placeholder_text("Pesquisar aplicativos...")
        .halign(Align::Center)
        .width_request(500)
        .build();

    let apps_flow_box = FlowBox::builder()
        .halign(Align::Center)
        .valign(Align::Start)
        .max_children_per_line(8)
        .min_children_per_line(5)
        .selection_mode(SelectionMode::None)
        .row_spacing(20)
        .selection_mode(SelectionMode::None)
        .width_request(600)   
        .build();

    apps_flow_box.set_margin_bottom(100);

    // Usando gio de dentro do gtk4 explicitamente
    let apps = gio::AppInfo::all();
    
    for app_info in apps {
        // Forçamos o tipo para o compilador não se perder
        let app_info: gio::AppInfo = app_info;

        if app_info.should_show() {
            let item_vbox = Box::new(Orientation::Vertical, 10);
            item_vbox.set_width_request(50);

            if let Some(icon) = app_info.icon() {
                let image = Image::from_gicon(&icon);
                image.set_pixel_size(96);
                item_vbox.append(&image);
            }

            let label = Label::builder()
                .label(app_info.name().as_str())
                .ellipsize(gtk4::pango::EllipsizeMode::End)
                .halign(Align::Center)
                .build();
            item_vbox.append(&label);

            let btn = Button::builder()
                .child(&item_vbox)
                .css_classes(vec!["app-item".to_string()])
                .has_frame(false)
                .build();

            let app_info_clone = app_info.clone();
            let window_clone = window.clone();
            btn.connect_clicked(move |_| {
                // Launch context como None
                let _ = app_info_clone.launch(&[], None::<&gio::AppLaunchContext>);
                window_clone.close();
            });

            // No GTK4 FlowBox, usamos insert para adicionar widgets
            apps_flow_box.insert(&btn, -1);
        }
    }

    let flow_box_clone = apps_flow_box.clone();

    // 1. Lógica de Filtragem
    search_bar.connect_search_changed(move |entry| {
        let text = entry.text().to_lowercase(); // Texto digitado em minúsculo
        
        flow_box_clone.set_filter_func(move |child| {
            // No GTK4, o 'child' do FlowBox é um FlowBoxChild que contém nosso Button
            if let Some(button) = child.child().and_downcast_ref::<gtk4::Button>() {
                // Pegamos o Label dentro do botão (que contém o nome do app)
                if let Some(item_vbox) = button.child().and_downcast_ref::<gtk4::Box>() {
                    // O segundo item do vbox (índice 1) é o nosso Label
                    if let Some(label) = item_vbox.last_child().and_downcast_ref::<gtk4::Label>() {
                        let app_name = label.label().to_lowercase();
                        return app_name.contains(&text); // Retorna true se o nome contém a busca
                    }
                }
            }
            true // Se não conseguir ler, mantém visível
        });
    });

    let scrolled = ScrolledWindow::builder()
        .child(&apps_flow_box)
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .vexpand(true)               // Faz o scroll ocupar o espaço vertical restante
        .propagate_natural_height(true)
        .min_content_height(200)
        .width_request(600) 
        .build();

    scrolled.set_margin_bottom(40);

    main_vbox.append(&search_bar);
    main_vbox.append(&workspaces_container);
    main_vbox.append(&scrolled); 

    overlay.add_overlay(&main_vbox);

    let controller = gtk4::EventControllerKey::new();
    controller.connect_key_pressed(|_, key, _, _| {
        if key == gtk4::gdk::Key::Escape {
            std::process::exit(0);
        }
        gtk4::glib::Propagation::Proceed
    });

    overlay.add_overlay(&main_vbox);
    window.add_controller(controller);
    window.set_child(Some(&overlay));
    window.present();
}
